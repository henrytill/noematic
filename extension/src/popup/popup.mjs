import { kSchemaVersion } from '../common/common.mjs';

/**
 * @typedef {import('../common/types.js').State} State
 */

const handleSearch = () => {
    chrome.tabs.create({ url: '/search/index.html' });
    window.close();
};

/**
 * @param {chrome.tabs.Tab} tab
 * @returns {Promise<boolean>}
 */
const checkContentScriptActive = async (tab) => {
    if (tab.id === undefined) {
        return false;
    }
    try {
        await chrome.tabs.sendMessage(tab.id, { action: 'ping' });
        return true;
    } catch (_) {
        return false;
    }
};

/**
 * @param {chrome.tabs.Tab} tab
 * @returns {Promise<chrome.scripting.InjectionResult<any>[]>}
 */
const installContentScript = (tab) => {
    if (tab.id === undefined) {
        return Promise.reject(new Error('No tab id'));
    }
    const isChrome = Object.prototype.hasOwnProperty.call(window, 'browser') === false;
    const files = isChrome ? ['./content/content.js'] : ['../content/content.js'];
    return chrome.scripting.executeScript({
        target: { tabId: tab.id },
        files,
    });
};

/**
 * @param {chrome.tabs.Tab} tab
 */
const handleSave = (tab) => {
    if (tab.id === undefined) {
        throw Error('No tab id');
    }
    const tabId = tab.id; // for TypeScript
    const message = { version: kSchemaVersion, action: 'saveRequest', payload: { url: tab.url } };
    checkContentScriptActive(tab)
        .then((isActive) => (isActive ? Promise.resolve([]) : installContentScript(tab)))
        .then((_) => chrome.tabs.sendMessage(tabId, message))
        .then((response) => console.log('response', response));
};

/**
 * Abbreviates a string to a given length if it is longer than the length.
 * @param {string} str
 * @param {number} length
 * @returns {string}
 */
const abbreviate = (str, length) => {
    if (str.length <= length) {
        return str;
    } else {
        return str.slice(0, length - 3) + '...';
    }
};

/**
 * @param {State} state
 * @returns {void}
 */
const updateView = (state) => {
    const mainDiv = document.getElementById('main');
    if (mainDiv === null) {
        throw new Error('No main div');
    }
    const originDiv = document.getElementById('origin');
    if (originDiv === null) {
        throw new Error('No origin');
    }
    const saveButton = /** @type {HTMLButtonElement} */ (document.getElementById('save'));
    if (saveButton === null) {
        throw new Error('No save button');
    }

    if (state.url !== null) {
        originDiv.textContent = abbreviate(state.url.origin, 50);
    } else {
        saveButton.disabled = true;
        mainDiv.removeChild(originDiv);
    }
};

/**
 * @param {State} state
 * @returns {void}
 */
const addListeners = (state) => {
    document.getElementById('cancel')?.addEventListener('click', () => window.close());
    document.getElementById('search')?.addEventListener('click', handleSearch);
    document.getElementById('save')?.addEventListener('click', handleSave.bind(null, state.tab));
};

/**
 * @param {URL} url
 * @returns {boolean}
 */
const isWeb = (url) => ['http:', 'https:'].includes(url.protocol);

/**
 * @returns {Promise<void>}
 */
const main = async () => {
    const tabs = await chrome.tabs.query({ active: true, currentWindow: true });
    if (tabs.length !== 1) {
        throw new Error(`Expected 1 active tab, got ${tabs.length}`);
    }
    const activeTab = tabs[0];
    if (activeTab.url == undefined) {
        throw new Error('No active tab url');
    }
    const url = new URL(activeTab.url);
    const state = { url: isWeb(url) ? url : null, tab: activeTab };
    addListeners(state);
    updateView(state);
};

main().catch((err) => console.error(err));
