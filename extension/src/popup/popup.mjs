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
  try {
    if (!tab.id) {
      throw Error('No tab id');
    }
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
  return chrome.scripting.executeScript({
    target: { tabId: tab.id },
    files: ['./content/content.js'],
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
 * @param {URL} url
 * @returns {HTMLDivElement}
 */
const createPanel = (url) => {
  const panel = document.createElement('div');
  panel.id = 'origin';
  panel.className = 'panel';
  panel.textContent = abbreviate(url.toString(), 50);
  return panel;
};

/**
 * @param {{id: string, className: string, textContent: string, disabled?: boolean}} attributes
 * @param {EventListener} onClick
 * @returns {HTMLButtonElement}
 */
const createButton = (attributes, onClick) => {
  const { id, className, textContent } = attributes;
  const button = document.createElement('button');
  button.id = id;
  button.className = className;
  button.textContent = textContent;
  if (attributes.disabled) {
    button.disabled = attributes.disabled;
  }
  button.addEventListener('click', onClick);
  return button;
};

/**
 * Creates a footer with buttons and appends it to the main div.
 * @param {State} state
 * @param {HTMLElement} mainDiv
 * @returns {void}
 */
const createFooter = (state, mainDiv) => {
  const cancel = createButton(
    { id: 'cancel', className: 'footer-button', textContent: 'Cancel' },
    window.close,
  );
  const search = createButton(
    { id: 'search', className: 'footer-button', textContent: 'Open Search...' },
    handleSearch,
  );
  const save = createButton(
    { id: 'save', className: 'footer-button', textContent: 'Save', disabled: state.url === null },
    handleSave.bind(null, state.tab),
  );
  const footer = document.createElement('footer');
  [cancel, search, save].forEach((elt) => footer.appendChild(elt));
  mainDiv.appendChild(footer);
};

/**
 * @param {State} state
 * @returns {void}
 */
const render = (state) => {
  const mainDiv = document.getElementById('main');
  if (!mainDiv) {
    throw new Error('No main div');
  }
  if (state.url !== null) {
    const panel = createPanel(state.url);
    mainDiv.appendChild(panel);
  }
  createFooter(state, mainDiv);
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
  render(state);
};

main().catch((err) => console.error(err));
