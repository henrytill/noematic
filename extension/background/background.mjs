import { NATIVE_MESSAGING_HOST, SCHEMA_VERSION } from '../common/common.mjs';
import { MessageCollector } from '../common/message-collector.mjs';

/**
 * @typedef {Map<import('../common/types.js').UUID, MessageCollector>} ResponderMap
 */

/**
 * @param {ResponderMap} responderMap
 * @param {import('../common/types.js').Response} message
 * @returns {void}
 */
const nativeListener = (responderMap, message) => {
  const correlationId = message.correlationId;
  const collector = responderMap.get(correlationId);
  if (collector === undefined) {
    console.error('No collector for correlation id', correlationId);
    return;
  }
  const collected = collector.push(message);
  if (collected) {
    responderMap.delete(correlationId);
  }
};

/**
 * @param {ResponderMap} responderMap
 * @param {chrome.runtime.Port} hostPort
 * @param {any} message
 * @param {chrome.runtime.MessageSender} _sender
 * @param {import('../common/types.js').Responder} sendResponse
 * @returns {boolean | undefined}
 */
const runtimeListener = (responderMap, hostPort, message, _sender, sendResponse) => {
  const correlationId = crypto.randomUUID();
  message.correlationId = correlationId;
  console.log('request', message);
  responderMap.set(correlationId, new MessageCollector(correlationId, sendResponse));
  hostPort.postMessage(message);
  return true;
};

/**
 * @returns {Promise<chrome.tabs.Tab>}
 */
const getActiveTab = async () => {
  const tabs = await chrome.tabs.query({ active: true, currentWindow: true });
  if (tabs.length !== 1) {
    throw new Error(`Expected 1 active tab, got ${tabs.length}`);
  }
  return tabs[0];
};

/**
 * @param {chrome.tabs.Tab} tab
 * @returns {Promise<{tab: chrome.tabs.Tab, active: boolean}>}
 */
const checkContentScriptActive = async (tab) => {
  if (tab.id === undefined) {
    return { tab, active: false };
  }
  try {
    await chrome.tabs.sendMessage(tab.id, { action: 'ping' });
    return { tab, active: true };
  } catch (_) {
    return { tab, active: false };
  }
};

/**
 * @param {{tab: chrome.tabs.Tab, active: boolean}} tab
 * @returns {Promise<number>}
 */
const maybeInstallContentScript = async ({ tab, active }) => {
  if (tab.id === undefined) {
    throw new Error('No tab id');
  }
  const tabId = tab.id;
  if (active) {
    return tabId;
  }
  const isChrome = Object.prototype.hasOwnProperty.call(globalThis, 'browser') === false;
  const prefix = isChrome ? '.' : '..'; // This is garbage.
  const files = [`${prefix}/content/content.js`];
  await chrome.scripting.executeScript({
    target: { tabId },
    files,
  });
  return tabId;
};

/**
 * @param {string} _id
 * @param {chrome.bookmarks.BookmarkTreeNode} bookmark
 * @returns {void}
 */
const bookmarksOnCreatedListener = (_id, bookmark) => {
  console.log('bookmark', bookmark);
  const message = {
    version: SCHEMA_VERSION,
    action: 'saveRequest',
    payload: { url: bookmark.url, title: bookmark.title },
  };
  getActiveTab()
    .then((tab) => checkContentScriptActive(tab))
    .then((result) => maybeInstallContentScript(result))
    .then((tabId) => chrome.tabs.sendMessage(tabId, message))
    .then((response) => console.log('response', response));
};

/**
 * @param {ResponderMap} responderMap
 * @param {chrome.runtime.Port} hostPort
 * @param {string} _id
 * @param {chrome.bookmarks.BookmarkRemoveInfo} removeInfo
 * @returns {void}
 */
const bookmarksOnRemovedListener = (responderMap, hostPort, _id, removeInfo) => {
  const bookmark = removeInfo.node;
  const correlationId = crypto.randomUUID();
  responderMap.set(
    correlationId,
    new MessageCollector(correlationId, (response) => console.log('response', response)),
  );
  const message = {
    version: SCHEMA_VERSION,
    action: 'removeRequest',
    payload: { url: bookmark.url },
    correlationId,
  };
  hostPort.postMessage(message);
};

/** @type {ResponderMap} */
const responderMap = new Map();

const port = chrome.runtime.connectNative(NATIVE_MESSAGING_HOST);

port.onMessage.addListener(nativeListener.bind(null, responderMap));

port.onDisconnect.addListener((_port) => console.debug('Disconnected from native messaging host'));

chrome.runtime.onMessage.addListener(runtimeListener.bind(null, responderMap, port));

chrome.bookmarks.onCreated.addListener(bookmarksOnCreatedListener);

chrome.bookmarks.onRemoved.addListener(bookmarksOnRemovedListener.bind(null, responderMap, port));

console.debug('Noematic background handler installed');
