// eslint-env node

import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';

import { AutoInput, Channel, FileCell, Target, hash } from '@henrytill/incr';

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
 * Copy a file from source to target if source is newer than target.
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
        gecko: { id: 'henrytill@gmail.com' },
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
async function makeSources(sourceFiles, ctor) {
    /** @type {Sources} */
    const ret = {};
    for (const source of sourceFiles) {
        const file = await ctor(source);
        ret[file.key] = file;
    }
    return ret;
}

/**
 * Prefix each shared target
 *
 * @param {string[]} sharedPrefixes
 * @param {TargetDefs} sharedTargets
 * @returns {TargetDefs}
 */
function prefixSharedTargets(sharedPrefixes, sharedTargets) {
    /** @type {TargetDefs} */
    const ret = {};
    for (const [key, { compute, inputs }] of Object.entries(sharedTargets)) {
        for (const prefix of sharedPrefixes) {
            ret[path.join(prefix, key)] = { compute, inputs };
        }
    }
    return ret;
}

/**
 * @param {TargetDefs} targetDefs
 * @returns {Targets}
 */
function makeTargets(targetDefs) {
    /** @type {Targets} */
    const ret = {};
    for (const [key, { compute, inputs }] of Object.entries(targetDefs)) {
        const target = new Target(inputs, compute, key);
        ret[target.key] = target;
    }
    return ret;
}

const sourceFiles = [
    'src/background/background.mjs',
    'src/common/common.mjs',
    'src/content/content.js',
    'src/icons/noematic-48.png',
    'src/popup/popup.css',
    'src/popup/popup.html',
    'src/popup/popup.mjs',
    'src/search/index.css',
    'src/search/index.html',
    'src/search/search-result.mjs',
    'src/search/search.css',
    'src/search/search.html',
    'src/search/search.mjs',
    'src/search/shared.css',
    'src/manifest.json',
];

const sharedPrefixes = ['dist/chromium', 'dist/firefox'];

/**
 * @param {Sources} sources
 * @return {TargetDefs}
 */
const makeSharedTargets = (sources) => ({
    'background/background.mjs': {
        inputs: [sources['src/background/background.mjs']],
        compute: copy,
    },
    'common/common.mjs': {
        inputs: [sources['src/common/common.mjs']],
        compute: copy,
    },
    'content/content.js': {
        inputs: [sources['src/content/content.js']],
        compute: copy,
    },
    'icons/noematic-48.png': {
        inputs: [sources['src/icons/noematic-48.png']],
        compute: copy,
    },
    'popup/popup.css': {
        inputs: [sources['src/popup/popup.css']],
        compute: copy,
    },
    'popup/popup.html': {
        inputs: [sources['src/popup/popup.html']],
        compute: copy,
    },
    'popup/popup.mjs': {
        inputs: [sources['src/popup/popup.mjs']],
        compute: copy,
    },
    'search/index.css': {
        inputs: [sources['src/search/index.css']],
        compute: copy,
    },
    'search/index.html': {
        compute: copy,
        inputs: [sources['src/search/index.html']],
    },
    'search/search-result.mjs': {
        inputs: [sources['src/search/search-result.mjs']],
        compute: copy,
    },
    'search/search.css': {
        inputs: [sources['src/search/search.css']],
        compute: copy,
    },
    'search/search.html': {
        inputs: [sources['src/search/search.html']],
        compute: copy,
    },
    'search/search.mjs': {
        inputs: [sources['src/search/search.mjs']],
        compute: copy,
    },
    'search/shared.css': {
        inputs: [sources['src/search/shared.css']],
        compute: copy,
    },
});

/**
 * @param {Sources} sources
 * @return {TargetDefs}
 */
const makeChromiumTargets = (sources) => ({
    'dist/chromium/manifest.json': {
        inputs: [sources['src/manifest.json']],
        compute: copy,
    },
});

/**
 * @param {Sources} sources
 * @return {TargetDefs}
 */
const makeFirefoxTargets = (sources) => ({
    'dist/firefox/manifest.json': {
        inputs: [sources['src/manifest.json']],
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
async function buildGraph(ctor) {
    const sources = await makeSources(sourceFiles, ctor);
    const sharedTargets = makeSharedTargets(sources);
    const prefixedSharedTargets = prefixSharedTargets(sharedPrefixes, sharedTargets);
    const browserSpecificTargets = makeBrowserSpecificTargets(sources);
    const targetDefs = { ...prefixedSharedTargets, ...browserSpecificTargets };
    const targetNodes = makeTargets(targetDefs);
    return [sources, targetNodes];
}

/**
 * @returns {Promise<void>}
 */
async function watch() {
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
}

/**
 * @returns {Promise<void>}
 */
async function build() {
    const [_, targetNodes] = await buildGraph(FileCell.of);
    for (const target of Object.values(targetNodes)) {
        await target.compute().value;
    }
}

function main() {
    const args = process.argv.slice(2);
    const subcommand = args.shift();

    switch (subcommand) {
        case 'watch':
            watch();
            break;
        case 'build':
        default:
            build();
    }
}

main();
