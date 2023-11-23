// eslint-env node

import * as fs from 'node:fs';
import * as os from 'node:os';
import * as path from 'node:path';
import * as process from 'node:process';
import * as url from 'node:url';

const kProjectRoot = path.join(path.dirname(url.fileURLToPath(import.meta.url)), '..', '..');
const kHostRoot = path.join(kProjectRoot, 'host');

const kHostBinaryName = 'noematic';

/**
 * @typedef {Object} Manifest
 * @property {string} name
 * @property {string} description
 * @property {string?} path
 * @property {'stdio'} type
 * @property {string[]} allowed_origins
 */

/** @type {Manifest} */
const template = {
  name: 'com.github.henrytill.noematic',
  description: 'Search your backlog',
  path: null,
  type: 'stdio',
  allowed_origins: ['chrome-extension://gebmhafgijeggbfhdojjefpibglhdjhh/'],
};

/**
 * Find the host project directory.
 *
 * @returns {string}
 * @throws {Error} if the host project directory does not exist
 */
const getHostDir = () => {
  const hostDir = process.env.HOST_PROJECT_DIR || kHostRoot;
  // Check that the host project directory exists
  if (!fs.existsSync(hostDir)) {
    throw new Error(`Host project directory does not exist: ${hostDir}`);
  }
  return hostDir;
};

/**
 * Set the template's path property to the host binary path.
 *
 * @param {Manifest} manifest
 * @param {string} hostDir
 * @param {string} buildType
 * @returns {void}
 * @throws {Error} if the host binary does not exist
 */
const setHostBinaryPath = (manifest, hostDir, buildType) => {
  const hostBinaryPath = path.join(hostDir, 'target', buildType, kHostBinaryName);
  // Check that the host binary exists
  if (!fs.existsSync(hostBinaryPath)) {
    throw new Error(`Host binary does not exist: ${hostBinaryPath}`);
  }
  manifest.path = hostBinaryPath;
};

/**
 * Find the target directory.
 *
 * @returns {string}
 */
const getTargetDir = () => {
  let targetDir = kProjectRoot;
  if (os.platform() == 'linux') {
    targetDir = path.join(os.homedir(), '.config', 'chromium', 'NativeMessagingHosts');
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
  const buildType = process.env.BUILD_TYPE || 'debug';

  try {
    const hostDir = getHostDir();

    setHostBinaryPath(template, hostDir, buildType);

    const targetDir = getTargetDir();

    const { manifestPath, output } = writeManifest(template, targetDir);

    console.log(`Host manifest written to: ${manifestPath}`);
    console.log(`Host manifest contents:\n${output}`);
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
};

main();
