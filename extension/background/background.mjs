import { NATIVE_MESSAGING_HOST } from '../common/common.mjs';
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

/** @type {ResponderMap} */
const responderMap = new Map();

const port = chrome.runtime.connectNative(NATIVE_MESSAGING_HOST);

port.onMessage.addListener(nativeListener.bind(null, responderMap));

port.onDisconnect.addListener((_port) => console.debug('Disconnected from native messaging host'));

chrome.runtime.onMessage.addListener(runtimeListener.bind(null, responderMap, port));

console.debug('Noematic background handler installed');
