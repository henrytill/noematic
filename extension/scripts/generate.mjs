// eslint-env node

import * as fs from 'node:fs';
import * as path from 'node:path';
import * as os from 'node:os';
import * as process from 'node:process';
import * as url from 'node:url';

/**
 * @typedef {Object} Manifest
 * @property {string} name
 * @property {string} description
 * @property {string?} path
 * @property {string} type
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
 * @param {NodeJS.ProcessEnv} env
 * @param {string} dirname
 * @returns {string}
 */
const getHostDir = (env, dirname) => {
  const defaultHostDir = path.join(dirname, '..', '..', 'host');
  const hostDir = env.HOST_PROJECT_DIR || defaultHostDir;
  // Check that the host project directory exists
  if (!fs.existsSync(hostDir)) {
    throw new Error(`Host project directory does not exist: ${hostDir}`);
  }
  return hostDir;
};

/**
 * @param {Manifest} template
 * @param {string} hostDir
 * @returns {void}
 */
const setHostBinaryPath = (template, hostDir) => {
  const hostBinaryName = 'noematic';
  const hostBinaryPath = path.join(hostDir, 'target', 'debug', hostBinaryName);
  // Check that the host binary exists
  if (!fs.existsSync(hostBinaryPath)) {
    throw new Error(`Host binary does not exist: ${hostBinaryPath}`);
  }
  template.path = hostBinaryPath;
};

/**
 * @param {NodeJS.ProcessEnv} env
 * @returns {string}
 */
const getTargetDir = (env) => {
  const defaultTargetDir = path.join(os.homedir(), '.config', 'chromium', 'NativeMessagingHosts');
  const targetDir = env.NATIVE_MESSAGING_HOSTS_DIR || defaultTargetDir;
  return targetDir;
};

/**
 * @param {Manifest} template
 * @param {string} targetDir
 * @returns {string}
 */
const writeManifest = (template, targetDir) => {
  fs.mkdirSync(targetDir, { recursive: true });
  const manifestPath = path.join(targetDir, `${template.name}.json`);
  fs.writeFileSync(manifestPath, JSON.stringify(template, null, 2), 'utf-8');
  return manifestPath;
};

const main = () => {
  const env = process.env;
  const __dirname = path.dirname(url.fileURLToPath(import.meta.url));

  try {
    // Find the host project directory
    const hostDir = getHostDir(env, __dirname);
    console.log(`Using host project directory: ${hostDir}`);

    // Set the build type
    const buildType = env.BUILD_TYPE || 'debug';

    // Set the host binary path
    setHostBinaryPath(template, hostDir);
    console.log(`Using host binary: ${template.path}`);

    // Set the target directory
    const targetDir = getTargetDir(env);
    console.log(`Using target directory: ${targetDir}`);

    // Write the manifest, creating the target directory if necessary
    const manifestPath = writeManifest(template, targetDir);
    console.log(`Native Messaging Host Manifest has been written to ${manifestPath}`);
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
};

main();
