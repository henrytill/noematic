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
 * @property {string[]} allowed_extensions
 */

/** @type {Manifest} */
const template = {
    name: 'com.github.henrytill.noematic',
    description: 'Search your backlog',
    path: null,
    type: 'stdio',
    allowed_origins: ['chrome-extension://gebmhafgijeggbfhdojjefpibglhdjhh/'],
    allowed_extensions: ['henrytill@gmail.com'],
};

/**
 * Find the host project directory.
 *
 * @returns {string}
 * @throws {Error} if the host project directory does not exist
 */
function getHostDir() {
    const hostDir = process.env.HOST_PROJECT_DIR || kHostRoot;
    // Check that the host project directory exists
    if (!fs.existsSync(hostDir)) {
        throw new Error(`Host project directory does not exist: ${hostDir}`);
    }
    return hostDir;
}

/**
 * Set the template's path property to the host binary path.
 *
 * @param {Manifest} manifest
 * @param {string} hostDir
 * @param {string} buildType
 * @returns {void}
 * @throws {Error} if the host binary does not exist
 */
function setHostBinaryPath(manifest, hostDir, buildType) {
    const hostBinaryPath = path.join(hostDir, 'target', buildType, kHostBinaryName);
    // Check that the host binary exists
    if (!fs.existsSync(hostBinaryPath)) {
        throw new Error(`Host binary does not exist: ${hostBinaryPath}`);
    }
    manifest.path = hostBinaryPath;
}

/**
 * Find the target directory for Chromium.
 *
 * @returns {string}
 */
function getChromiumTargetDir() {
    let targetDir = kProjectRoot;
    if (os.platform() == 'linux') {
        targetDir = path.join(os.homedir(), '.config', 'chromium', 'NativeMessagingHosts');
    }
    return process.env.NATIVE_MESSAGING_HOSTS_DIR || targetDir;
}

/**
 * Find the target directory for Firefox.
 *
 * @returns {string}
 */
function getFirefoxTargetDir() {
    let targetDir = kProjectRoot;
    if (os.platform() == 'linux') {
        targetDir = path.join(os.homedir(), '.mozilla', 'native-messaging-hosts');
    }
    return process.env.NATIVE_MESSAGING_HOSTS_DIR || targetDir;
}

/**
 * @param {Manifest} manifest
 * @param {string} targetDir
 * @returns {{manifestPath: string, output: string}}
 */
function writeManifest(manifest, targetDir) {
    fs.mkdirSync(targetDir, { recursive: true });
    const manifestPath = path.join(targetDir, `${manifest.name}.json`);
    const output = JSON.stringify(manifest, null, 2);
    fs.writeFileSync(manifestPath, output, 'utf-8');
    return { manifestPath, output };
}

function main() {
    const buildType = process.env.BUILD_TYPE || 'debug';
    try {
        const hostDir = getHostDir();
        setHostBinaryPath(template, hostDir, buildType);
        {
            const chromiumTargetDir = getChromiumTargetDir();
            const { manifestPath, output } = writeManifest(template, chromiumTargetDir);
            console.log(`Chromium host manifest written to: ${manifestPath}`);
            console.log(`Chromium host manifest contents:\n${output}`);
        }
        {
            const firefoxTargetDir = getFirefoxTargetDir();
            const { manifestPath, output } = writeManifest(template, firefoxTargetDir);
            console.log(`Firefox host manifest written to: ${manifestPath}`);
            console.log(`Firefox host manifest contents:\n${output}`);
        }
    } catch (err) {
        console.error(err);
        process.exit(1);
    }
}

main();
