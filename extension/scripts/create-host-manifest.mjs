/* eslint-env node */

import * as fs from 'node:fs';
import * as os from 'node:os';
import * as path from 'node:path';
import * as process from 'node:process';
import * as url from 'node:url';

import { FIREFOX_ID } from './common.mjs';

const PROJECT_ROOT = path.join(path.dirname(url.fileURLToPath(import.meta.url)), '..', '..');
const HOST_BINARY_NAME = 'noematic';
const ALLOWED_ORIGIN = 'chrome-extension://gebmhafgijeggbfhdojjefpibglhdjhh/';
const NAME = 'com.github.henrytill.noematic';
const DESCRIPTION = 'Search your backlog';

/** @enum {number} */
const Browser = {
  Chromium: 0,
  Firefox: 1,
};

/**
 * @typedef {Object} Manifest
 * @property {string} name
 * @property {string} description
 * @property {string?} path
 * @property {'stdio'} type
 * @property {string[]} [allowed_origins]
 * @property {string[]} [allowed_extensions]
 */

/** @type {Manifest} */
const template = {
  name: NAME,
  description: DESCRIPTION,
  path: null,
  type: 'stdio',
};

/**
 * Find the project's target directory.
 *
 * @returns {string}
 * @throws {Error} if the target directory does not exist
 */
const getTargetDir = () => {
  const projectRoot = process.env.PROJECT_ROOT || PROJECT_ROOT;
  const targetDir = path.join(projectRoot, 'target');
  if (!fs.existsSync(targetDir)) {
    throw new Error(`Directory does not exist: ${targetDir}`);
  }
  return targetDir;
};

/**
 * Set the template's path property to the host binary path.
 *
 * @param {Manifest} template
 * @param {number} browser
 * @param {string} targetDir,
 * @param {string} buildType
 * @returns {Manifest}
 * @throws {Error} if the host binary does not exist
 */
const createManifest = (template, browser, targetDir, buildType) => {
  const ret = { ...template };
  const hostBinaryPath = path.join(targetDir, buildType, HOST_BINARY_NAME);
  // Check that the host binary exists
  if (!fs.existsSync(hostBinaryPath)) {
    throw new Error(`Host binary does not exist: ${hostBinaryPath}`);
  }
  ret.path = hostBinaryPath;
  switch (browser) {
    case Browser.Chromium:
      ret.allowed_origins = [ALLOWED_ORIGIN];
      break;
    case Browser.Firefox:
      ret.allowed_extensions = [FIREFOX_ID];
      break;
    default:
      throw new Error(`Unsupported browser: ${browser}`);
  }

  return ret;
};

/**
 * Find the target directory for Chromium.
 *
 * @returns {string}
 */
const getChromiumTargetDir = () => {
  let targetDir = PROJECT_ROOT;
  if (os.platform() == 'linux') {
    targetDir = path.join(os.homedir(), '.config', 'chromium', 'NativeMessagingHosts');
  }
  return process.env.NATIVE_MESSAGING_HOSTS_DIR || targetDir;
};

/**
 * Find the target directory for Firefox.
 *
 * @returns {string}
 */
const getFirefoxTargetDir = () => {
  let targetDir = PROJECT_ROOT;
  if (os.platform() == 'linux') {
    targetDir = path.join(os.homedir(), '.mozilla', 'native-messaging-hosts');
  }
  return process.env.NATIVE_MESSAGING_HOSTS_DIR || targetDir;
};

/**
 * @param {Manifest} manifest
 * @param {string} targetDir
 * @returns {{manifestPath: string, output: string}}
 */
const writeManifest = (manifest, targetDir) => {
  fs.mkdirSync(targetDir, { recursive: true });
  const manifestPath = path.join(targetDir, `${manifest.name}.json`);
  const output = JSON.stringify(manifest, null, 2);
  fs.writeFileSync(manifestPath, output, 'utf-8');
  return { manifestPath, output };
};

const main = () => {
  try {
    const targetDir = getTargetDir();
    const buildType = process.env.BUILD_TYPE || 'debug';
    {
      const manifest = createManifest(template, Browser.Chromium, targetDir, buildType);
      const chromiumTargetDir = getChromiumTargetDir();
      const { manifestPath, output } = writeManifest(manifest, chromiumTargetDir);
      console.log(`Chromium host manifest written to: ${manifestPath}`);
      console.log(`Chromium host manifest contents:\n${output}`);
    }
    {
      const manifest = createManifest(template, Browser.Firefox, targetDir, buildType);
      const firefoxTargetDir = getFirefoxTargetDir();
      const { manifestPath, output } = writeManifest(manifest, firefoxTargetDir);
      console.log(`Firefox host manifest written to: ${manifestPath}`);
      console.log(`Firefox host manifest contents:\n${output}`);
    }
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
};

main();
