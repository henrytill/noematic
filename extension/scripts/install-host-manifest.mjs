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
 * Find the host project directory.
 *
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
 * Set the template's path property to the host binary path.
 *
 * @param {Manifest} template
 * @param {string} hostDir
 * @param {string} buildType
 * @returns {void}
 */
const setHostBinaryPath = (template, hostDir, buildType) => {
  const hostBinaryName = 'noematic';
  const hostBinaryPath = path.join(hostDir, 'target', buildType, hostBinaryName);
  // Check that the host binary exists
  if (!fs.existsSync(hostBinaryPath)) {
    throw new Error(`Host binary does not exist: ${hostBinaryPath}`);
  }
  template.path = hostBinaryPath;
};

/**
 * Find the target directory.
 *
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
 * @returns {{manifestPath: string, output: string}}
 */
const writeManifest = (template, targetDir) => {
  fs.mkdirSync(targetDir, { recursive: true });
  const manifestPath = path.join(targetDir, `${template.name}.json`);
  const output = JSON.stringify(template, null, 2);
  fs.writeFileSync(manifestPath, output, 'utf-8');
  return { manifestPath, output };
};

const main = () => {
  const env = process.env;
  const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
  const buildType = env.BUILD_TYPE || 'debug';

  try {
    const hostDir = getHostDir(env, __dirname);

    setHostBinaryPath(template, hostDir, buildType);

    const targetDir = getTargetDir(env);

    const { manifestPath, output } = writeManifest(template, targetDir);

    console.log(`Manifest written to: ${manifestPath}`);
    console.log(`Manifest contents:\n${output}`);
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
};

main();
