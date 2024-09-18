import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';

import { AutoInput, Channel, FileCell, Target, hash } from '@henrytill/incr';

import { FIREFOX_ID } from './common.mjs';

/**
 * @template A, B
 * @typedef {import('@henrytill/incr').ComputeFunction<A, B>} ComputeFunction<A, B>
 */

/**
 * @template A
 * @typedef {import('@henrytill/incr').Node<A>} Node<A>
 */

/**
 * @typedef {import('@henrytill/incr').HashDigest} HashDigest
 */

/**
 * @typedef {Object} TargetDef
 * @property {import('@henrytill/incr').BuildNode[]} inputs
 * @property {import('@henrytill/incr').ComputeFunction<any, any>} compute
 */

/**
 * Copy a file from source to target.
 *
 * @this {Target}
 * @param {Node<HashDigest>} source
 * @returns {Promise<HashDigest>}
 */
async function copy(source) {
  await fs.mkdir(path.dirname(this.key), { recursive: true });
  console.debug('Copying', source.key, 'to', this.key);
  await fs.copyFile(source.key, this.key);
  const contents = await fs.readFile(this.key);
  return hash(contents);
}

/**
 * Create a Firefox Manifest V3 file from a Chromium Manifest V3 file.
 *
 * @this {Target}
 * @param {Node<HashDigest>} source
 * @returns {Promise<HashDigest>}
 */
async function generateFirefoxManifest(source) {
  const file = await fs.readFile(source.key, 'utf8');
  const manifest = JSON.parse(file);
  manifest.background.scripts = ['background/background.mjs'];
  manifest.browser_specific_settings = {
    gecko: { id: FIREFOX_ID },
  };
  delete manifest.background.service_worker;
  delete manifest.key;
  const contents = JSON.stringify(manifest, null, 2);
  await fs.mkdir(path.dirname(this.key), { recursive: true });
  await fs.writeFile(this.key, contents);
  return hash(contents);
}

/**
 * @typedef {(source: string) => Promise<FileCell | AutoInput>} FileConstructor
 * @typedef {Record<string, FileCell | AutoInput>} Sources
 * @typedef {Record<string, TargetDef>} TargetDefs
 * @typedef {Record<string, Target>} Targets
 */

/**
 * @param {string[]} sourceFiles
 * @param {FileConstructor} ctor
 * @return {Promise<Sources>}
 */
const makeSources = async (sourceFiles, ctor) => {
  /** @type {Sources} */
  const ret = {};
  for (const source of sourceFiles) {
    const file = await ctor(source);
    ret[file.key] = file;
  }
  return ret;
};

/**
 * Prefix each shared target
 *
 * @param {string[]} sharedPrefixes
 * @param {TargetDefs} sharedTargets
 * @returns {TargetDefs}
 */
const prefixSharedTargets = (sharedPrefixes, sharedTargets) => {
  /** @type {TargetDefs} */
  const ret = {};
  for (const [key, { compute, inputs }] of Object.entries(sharedTargets)) {
    for (const prefix of sharedPrefixes) {
      ret[path.join(prefix, key)] = { compute, inputs };
    }
  }
  return ret;
};

/**
 * @param {TargetDefs} targetDefs
 * @returns {Targets}
 */
const makeTargets = (targetDefs) => {
  /** @type {Targets} */
  const ret = {};
  for (const [key, { compute, inputs }] of Object.entries(targetDefs)) {
    const target = new Target(inputs, compute, key);
    ret[target.key] = target;
  }
  return ret;
};

const sourceFiles = [
  'extension/background/background.event.mjs',
  'extension/background/background.worker.mjs',
  'extension/common/common.mjs',
  'extension/content/content.js',
  'extension/icons/noematic-48.png',
  'extension/popup/popup.css',
  'extension/popup/popup.html',
  'extension/popup/popup.mjs',
  'extension/search/index.css',
  'extension/search/index.html',
  'extension/search/search-result.mjs',
  'extension/search/search.css',
  'extension/search/search.html',
  'extension/search/search.mjs',
  'extension/search/shared.css',
  'extension/manifest.json',
];

/**
 * @param {Sources} sources
 * @return {TargetDefs}
 */
const makeSharedTargets = (sources) => ({
  'common/common.mjs': {
    inputs: [sources['extension/common/common.mjs']],
    compute: copy,
  },
  'content/content.js': {
    inputs: [sources['extension/content/content.js']],
    compute: copy,
  },
  'icons/noematic-48.png': {
    inputs: [sources['extension/icons/noematic-48.png']],
    compute: copy,
  },
  'popup/popup.css': {
    inputs: [sources['extension/popup/popup.css']],
    compute: copy,
  },
  'popup/popup.html': {
    inputs: [sources['extension/popup/popup.html']],
    compute: copy,
  },
  'popup/popup.mjs': {
    inputs: [sources['extension/popup/popup.mjs']],
    compute: copy,
  },
  'search/index.css': {
    inputs: [sources['extension/search/index.css']],
    compute: copy,
  },
  'search/index.html': {
    compute: copy,
    inputs: [sources['extension/search/index.html']],
  },
  'search/search-result.mjs': {
    inputs: [sources['extension/search/search-result.mjs']],
    compute: copy,
  },
  'search/search.css': {
    inputs: [sources['extension/search/search.css']],
    compute: copy,
  },
  'search/search.html': {
    inputs: [sources['extension/search/search.html']],
    compute: copy,
  },
  'search/search.mjs': {
    inputs: [sources['extension/search/search.mjs']],
    compute: copy,
  },
  'search/shared.css': {
    inputs: [sources['extension/search/shared.css']],
    compute: copy,
  },
});

/**
 * @param {Sources} sources
 * @return {TargetDefs}
 */
const makeChromiumTargets = (sources) => ({
  'dist/chromium/background/background.mjs': {
    inputs: [sources['extension/background/background.worker.mjs']],
    compute: copy,
  },
  'dist/chromium/manifest.json': {
    inputs: [sources['extension/manifest.json']],
    compute: copy,
  },
});

/**
 * @param {Sources} sources
 * @return {TargetDefs}
 */
const makeFirefoxTargets = (sources) => ({
  'dist/firefox/background/background.mjs': {
    inputs: [sources['extension/background/background.event.mjs']],
    compute: copy,
  },
  'dist/firefox/manifest.json': {
    inputs: [sources['extension/manifest.json']],
    compute: generateFirefoxManifest,
  },
});

/**
 * @param {Sources} sources
 * @return {TargetDefs}
 */
const makeBrowserSpecificTargets = (sources) => ({
  ...makeChromiumTargets(sources),
  ...makeFirefoxTargets(sources),
});

/**
 * @param {FileConstructor} ctor
 * @returns {Promise<[Sources, Targets]>}
 */
const buildGraph = async (ctor) => {
  const sources = await makeSources(sourceFiles, ctor);
  const prefixes = ['dist/chromium', 'dist/firefox'];
  const sharedTargets = makeSharedTargets(sources);
  const prefixedSharedTargets = prefixSharedTargets(prefixes, sharedTargets);
  const manifestTargets = makeBrowserSpecificTargets(sources);
  const targets = makeTargets({ ...prefixedSharedTargets, ...manifestTargets });
  return [sources, targets];
};

/**
 * @returns {Promise<void>}
 */
const watch = async () => {
  const notifications = new Channel();
  const consumer = (async () => {
    for await (const notification of notifications.receive()) {
      if (notification === undefined) break;
      const { filename } = notification;
      console.log('consumer:', filename, 'updated');
    }
  })();
  /** @type {FileConstructor} */
  const ctor = (source) => AutoInput.of(source, notifications);
  const [sources, _] = await buildGraph(ctor);
  process.on('SIGINT', () => {
    notifications.close();
    for (const source of Object.values(sources)) {
      assert.ok(source instanceof AutoInput);
      source.close();
    }
    console.log('\nExiting...');
  });
  await consumer;
};

/**
 * @returns {Promise<void>}
 */
const build = async () => {
  const [_, targetNodes] = await buildGraph(FileCell.of);
  for (const target of Object.values(targetNodes)) {
    await target.compute().value;
  }
};

/**
 * @returns {Promise<void>}
 */
const main = async () => {
  const args = process.argv.slice(2);
  const subcommand = args.shift();

  switch (subcommand) {
    case 'watch':
      await watch();
      break;
    case 'build':
    default:
      await build();
  }
};

main();
