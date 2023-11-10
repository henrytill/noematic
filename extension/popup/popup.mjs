/**
 * @typedef {import('../common/common.ts').State} State
 */

/**
 * @param {URL} url
 * @returns {boolean}
 */
const isWebUrl = (url) => {
  return ['http:', 'https:'].includes(url.protocol);
};

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
    return Promise.reject(Error('No tab id'));
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
  const message = { action: 'saveRequest', payload: { url: tab.url } };
  checkContentScriptActive(tab)
    .then((isActive) => (isActive ? Promise.resolve([]) : installContentScript(tab)))
    .then((_) => chrome.tabs.sendMessage(tabId, message))
    .then((response) => console.log('response', response));
};

/**
 * @param {URL} url
 * @returns {HTMLDivElement}
 */
const createPanel = (url) => {
  const panel = document.createElement('div');
  panel.id = 'origin';
  panel.className = 'panel';
  panel.textContent = url.toString();
  return panel;
};

/**
 * @param {string} id
 * @param {string} className
 * @param {string} textContent
 * @param {EventListener} onClick
 * @returns {HTMLButtonElement}
 */
const createButton = (id, className, textContent, onClick) => {
  const button = document.createElement('button');
  button.id = id;
  button.className = className;
  button.textContent = textContent;
  button.addEventListener('click', onClick);
  return button;
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
  if (isWebUrl(state.url)) {
    const panel = createPanel(state.url);
    mainDiv.appendChild(panel);
  }
  const footer = document.createElement('footer');
  const cancelButton = createButton('cancel', 'footer-button', 'Cancel', window.close);
  const openSearchButton = createButton('search', 'footer-button', 'Open Search...', handleSearch);
  const saveButton = createButton(
    'save',
    'footer-button',
    'Save',
    handleSave.bind(null, state.tab),
  );
  saveButton.disabled = !isWebUrl(state.url);
  footer.appendChild(cancelButton);
  footer.appendChild(openSearchButton);
  footer.appendChild(saveButton);
  mainDiv.appendChild(footer);
};

/**
 * @returns {Promise<void>}
 */
const main = async () => {
  const tabs = await chrome.tabs.query({ active: true, currentWindow: true });
  if (tabs.length === 0) {
    throw new Error('No active tab');
  }
  const activeTab = tabs[0];
  if (!activeTab.url) {
    throw new Error('No active tab url');
  }
  const url = new URL(activeTab.url);
  const state = { url: url, tab: activeTab };
  render(state);
};

main().catch((err) => console.error(err));
