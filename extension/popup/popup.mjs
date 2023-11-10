const isWebUrl = (url) => {
  return ['http:', 'https:'].includes(url.protocol);
};

const handleSearch = () => {
  chrome.tabs.create({ url: '/search/index.html' });
  window.close();
};

const checkContentScriptActive = async (tab) => {
  try {
    await chrome.tabs.sendMessage(tab.id, { action: 'ping' });
    return true;
  } catch (_) {
    return false;
  }
};

const installContentScript = (tab) => {
  return chrome.scripting.executeScript({
    target: { tabId: tab.id },
    files: ['./content/content.bc.js'],
  });
};

const handleSave = (tab) => {
  console.log('tab', tab);
  const message = { action: 'saveRequest', payload: { url: tab.url } };
  checkContentScriptActive(tab)
    .then((isActive) => (isActive ? Promise.resolve([]) : installContentScript(tab)))
    .then((_) => chrome.tabs.sendMessage(tab.id, message))
    .then((response) => console.log(response));
};

const createPanel = (url) => {
  const panel = document.createElement('div');
  panel.id = 'origin';
  panel.className = 'panel';
  panel.textContent = url;
  return panel;
};

const createButton = (id, className, textContent, onClick) => {
  const button = document.createElement('button');
  button.id = id;
  button.className = className;
  button.textContent = textContent;
  button.addEventListener('click', onClick);
  return button;
};

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

const main = async () => {
  const tabs = await chrome.tabs.query({ active: true, currentWindow: true });
  if (tabs.length === 0) {
    throw new Error('No active tab');
  }
  const activeTab = tabs[0];
  const url = new URL(activeTab.url);
  const state = { url: url, tab: activeTab };
  render(state);
};

main().catch((err) => console.error(err));
