/**
 * @typedef {import('../common/types.js').Responder} Responder
 * @typedef {import('../common/types.js').ResponderMap} ResponderMap
 */

const NATIVE_MESSAGING_HOST = 'com.github.henrytill.noematic';

/**
 * @param {ResponderMap} responderMap
 * @param {any} message
 * @returns {void}
 */
function handleHostMessage(responderMap, message) {
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
    console.log('response', message);
}

/**
 * @param {chrome.runtime.Port} _port
 * @returns {void}
 */
function handleHostDisconnect(_port) {
    console.debug('Disconnected from native messaging host');
}

/**
 * @param {string} application
 * @param {ResponderMap} responderMap
 * @returns {chrome.runtime.Port}
 */
function connectHost(application, responderMap) {
    const port = chrome.runtime.connectNative(application);
    port.onMessage.addListener(handleHostMessage.bind(null, responderMap));
    port.onDisconnect.addListener(handleHostDisconnect);
    return port;
}

/**
 * @param {ResponderMap} responderMap
 * @param {chrome.runtime.Port} hostPort
 * @param {any} request
 * @param {chrome.runtime.MessageSender} _sender
 * @param {Responder} sendResponse
 * @returns {boolean | undefined}
 */
function messageListener(responderMap, hostPort, request, _sender, sendResponse) {
    const correlationId = crypto.randomUUID();
    request.correlationId = correlationId;
    console.log('request', request);
    responderMap.set(correlationId, sendResponse);
    hostPort.postMessage(request);
    return true;
}

/**
 * @returns {void}
 */
function main() {
    /** @type {ResponderMap} */
    const responderMap = new Map();
    const hostPort = connectHost(NATIVE_MESSAGING_HOST, responderMap);
    chrome.runtime.onMessage.addListener(messageListener.bind(null, responderMap, hostPort));
    console.debug('Noematic background handler installed');
}

main();
