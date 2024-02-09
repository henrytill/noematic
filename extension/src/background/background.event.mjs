import { NATIVE_MESSAGING_HOST } from '../common/common.mjs';

/**
 * @typedef {import('../common/types.js').Responder} Responder
 */

/**
 * @param {any} request
 * @param {chrome.runtime.MessageSender} _sender
 * @param {Responder} sendResponse
 * @returns {boolean | undefined}
 */
function messageListener(request, _sender, sendResponse) {
    request.correlationId = crypto.randomUUID();
    console.log('request', request);
    chrome.runtime.sendNativeMessage(NATIVE_MESSAGING_HOST, request).then((message) => {
        sendResponse(message);
        console.log('response', message);
    });
    return true;
}

/**
 * @returns {void}
 */
function main() {
    chrome.runtime.onMessage.addListener(messageListener);
    console.debug('Noematic background handler installed');
}

main();
