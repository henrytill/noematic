#!/usr/bin/env node

const fs = require('fs');
const os = require('os');
const path = require('path');
const process = require('process');

const FIREFOX_ID = 'henrytill@gmail.com';
const PREFIX = path.join(path.dirname(__filename), '..');
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
 * Find the project's install prefix.
 *
 * @returns {string}
 * @throws {Error} if the directory does not exist
 */
const getPrefix = () => {
  const ret = PREFIX;
  if (!fs.existsSync(ret)) {
    throw new Error(`Directory does not exist: ${ret}`);
  }
  return ret;
};

/**
 * Set the template's path property to the host binary path.
 *
 * @param {Manifest} template
 * @param {number} browser
 * @param {string} prefix,
 * @returns {Manifest}
 * @throws {Error} if the host binary does not exist
 */
const createManifest = (template, browser, prefix) => {
  const ret = { ...template };
  const hostBinaryPath = path.join(prefix, 'bin', HOST_BINARY_NAME);
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
  const ret = process.env.NATIVE_MESSAGING_HOSTS_DIR;
  if (ret !== undefined) {
    return ret;
  }
  switch (os.platform()) {
    case 'linux':
      return path.join(os.homedir(), '.config', 'chromium', 'NativeMessagingHosts');
    default:
      return process.cwd();
  }
};

/**
 * Find the target directory for Firefox.
 *
 * @returns {string}
 */
const getFirefoxTargetDir = () => {
  const ret = process.env.NATIVE_MESSAGING_HOSTS_DIR;
  if (ret !== undefined) {
    return ret;
  }
  switch (os.platform()) {
    case 'linux':
      return path.join(os.homedir(), '.mozilla', 'native-messaging-hosts');
    default:
      return process.cwd();
  }
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

/**
 * @typedef {Object} Args
 * @property {string | null} prefix
 */

/**
 * @param {string[]} argv
 * @returns {Args}
 */
const parseArgv = (argv) => {
  /** @type {Args} */
  const ret = {
    prefix: null,
  };
  argv = argv.slice(2);
  for (let i = 0; i < argv.length; ++i) {
    const arg = argv[i];
    switch (arg) {
      case '--prefix':
        if (i + 1 < argv.length) {
          ret.prefix = argv[++i];
        } else {
          console.error('Error: --prefix requires a directory path');
          process.exit(1);
        }
        break;
      default:
        console.warn(`Warning: Unknown argument '${arg}'`);
    }
  }
  return ret;
};

const main = () => {
  const args = parseArgv(process.argv);
  try {
    const targetDir = args.prefix || getPrefix();
    {
      const manifest = createManifest(template, Browser.Chromium, targetDir);
      const chromiumTargetDir = getChromiumTargetDir();
      const { manifestPath, output } = writeManifest(manifest, chromiumTargetDir);
      console.log(`Chromium host manifest written to: ${manifestPath}`);
      console.log(`Chromium host manifest contents:\n${output}`);
    }
    {
      const manifest = createManifest(template, Browser.Firefox, targetDir);
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
