import * as common from '../common/common.mjs';
import { SCHEMA_VERSION } from '../common/common.mjs';

/**
 * @typedef {import('../common/types.js').State} State
 */

/**
 * @returns {void}
 */
const handleSearch = () => {
  chrome.tabs.create({ url: '/search/index.html' });
  window.close();
};

/**
 * @param {chrome.tabs.Tab} tab
 * @returns {Promise<boolean>}
 */
const checkContentScriptActive = async ({ id }) => {
  if (id === undefined) {
    return false;
  }
  try {
    await chrome.tabs.sendMessage(id, { action: 'ping' });
    return true;
  } catch (_) {
    return false;
  }
};

/**
 * @param {chrome.tabs.Tab} tab
 * @returns {Promise<chrome.scripting.InjectionResult<any>[]>}
 */
const installContentScript = ({ id }) => {
  if (id === undefined) {
    return Promise.reject(new Error('No tab id'));
  }
  const isChrome = Object.prototype.hasOwnProperty.call(window, 'browser') === false;
  const prefix = isChrome ? '.' : '..'; // This is garbage.
  const files = [`${prefix}/content/content.js`];
  return chrome.scripting.executeScript({
    target: { tabId: id },
    files,
  });
};

/**
 * @param {chrome.tabs.Tab} tab
 * @returns {void}
 */
const handleSave = (tab) => {
  if (tab.id === undefined) {
    throw Error('No tab id');
  }
  const tabId = tab.id; // for TypeScript
  const message = { version: SCHEMA_VERSION, action: 'saveRequest', payload: { url: tab.url } };
  checkContentScriptActive(tab)
    .then((isActive) => (isActive ? Promise.resolve([]) : installContentScript(tab)))
    .then((_) => chrome.tabs.sendMessage(tabId, message))
    .then((response) => console.log('response', response));
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
  if (state.url === null) {
    saveButton.disabled = true;
    mainDiv.removeChild(originDiv);
    return;
  }
  originDiv.textContent = common.abbreviate(state.url.toString(), 50);
};

/**
 * @param {State} state
 * @returns {void}
 */
const addListeners = (state) => {
  const handleCancel = () => window.close();
  document.getElementById('cancel')?.addEventListener('click', handleCancel);
  document.getElementById('search')?.addEventListener('click', handleSearch);
  document.getElementById('save')?.addEventListener('click', handleSave.bind(null, state.tab));
};

/**
 * @returns {Promise<void>}
 */
const main = async () => {
  const tabs = await chrome.tabs.query({ active: true, currentWindow: true });
  if (tabs.length !== 1) {
    throw new Error(`Expected 1 active tab, got ${tabs.length}`);
  }
  const tab = tabs[0];
  if (tab.url == undefined) {
    throw new Error('No active tab url');
  }
  /** @type {URL?} */
  let url = new URL(tab.url);
  url = ['http:', 'https:'].includes(url.protocol) ? url : null;
  const state = { url, tab };
  addListeners(state);
  updateView(state);
};

main().catch((err) => console.error(err));
