import { NATIVE_MESSAGING_HOST } from '../common/common.mjs';

/**
 * @typedef {import('../common/types.js').Responder} Responder
 * @typedef {import('../common/types.js').ResponderMap} ResponderMap
 */

/**
 * @param {ResponderMap} responderMap
 * @param {any} message
 * @returns {void}
 */
const handleHostMessage = (responderMap, message) => {
  const correlationId = message.correlationId;
  if (correlationId === undefined) {
    console.error('No correlation id in message', message);
    return;
  }
  const sendResponse = responderMap.get(correlationId);
  if (sendResponse === undefined) {
    console.error('No sendResponse function for correlation id', correlationId);
    return;
  }
  responderMap.delete(correlationId);
  sendResponse(message);
  console.log('response', message);
};

/**
 * @param {chrome.runtime.Port} _port
 * @returns {void}
 */
const handleHostDisconnect = (_port) => {
  console.debug('Disconnected from native messaging host');
};

/**
 * @param {string} application
 * @param {ResponderMap} responderMap
 * @returns {chrome.runtime.Port}
 */
const connectHost = (application, responderMap) => {
  const port = chrome.runtime.connectNative(application);
  port.onMessage.addListener(handleHostMessage.bind(null, responderMap));
  port.onDisconnect.addListener(handleHostDisconnect);
  return port;
};

/**
 * @param {ResponderMap} responderMap
 * @param {chrome.runtime.Port} hostPort
 * @param {any} request
 * @param {chrome.runtime.MessageSender} _sender
 * @param {Responder} sendResponse
 * @returns {boolean | undefined}
 */
const messageListener = (responderMap, hostPort, request, _sender, sendResponse) => {
  const correlationId = crypto.randomUUID();
  request.correlationId = correlationId;
  console.log('request', request);
  responderMap.set(correlationId, sendResponse);
  hostPort.postMessage(request);
  return true;
};

/**
 * @returns {void}
 */
const main = () => {
  /** @type {ResponderMap} */
  const responderMap = new Map();
  const hostPort = connectHost(NATIVE_MESSAGING_HOST, responderMap);
  chrome.runtime.onMessage.addListener(messageListener.bind(null, responderMap, hostPort));
  console.debug('Noematic background handler installed');
};

main();
