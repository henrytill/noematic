/**
 * @typedef {import('../common/types.js').UUID} UUID
 * @typedef {import('../common/types.js').Responder} Responder
 * @typedef {import('../common/types.js').ResponderMap} ResponderMap
 */

const kNativeMessagingHost = 'com.github.henrytill.noematic';

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
  const response = responderMap.get(correlationId);
  if (response === undefined) {
    console.error('No response handler for correlation id', correlationId);
    return;
  }
  responderMap.delete(correlationId);
  response(message);
};

/**
 * @param {chrome.runtime.Port} _
 */
const handleHostDisconnect = (_) => {
  console.debug('Disconnected from native messaging host');
};

/**
 * @param {ResponderMap} responderMap
 * @returns {chrome.runtime.Port}
 */
const connectHost = (responderMap) => {
  const port = chrome.runtime.connectNative(kNativeMessagingHost);
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
 * @returns {boolean}
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
  const hostPort = connectHost(responderMap);
  chrome.runtime.onMessage.addListener(messageListener.bind(null, responderMap, hostPort));
  console.debug('Noematic background handler installed');
};

main();
