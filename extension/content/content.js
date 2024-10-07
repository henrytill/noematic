/**
 * @typedef {import('../common/types.js').Responder} Responder
 */

/**
 * @param {any} request
 * @param {chrome.runtime.MessageSender} _sender
 * @param {Responder} sendResponse
 * @returns {boolean}
 */
const handleSaveRequests = (request, _sender, sendResponse) => {
  request.payload.innerText = document.body.innerText;
  chrome.runtime.sendMessage(request).then((response) => {
    response.action = 'saveResponse';
    sendResponse(response);
  });
  return true;
};

/**
 * @param {any} _request
 * @param {chrome.runtime.MessageSender} _sender
 * @param {Responder} sendResponse
 * @returns {boolean}
 */
const handlePingRequests = (_request, _sender, sendResponse) => {
  sendResponse({ action: 'pong' });
  return true;
};

/**
 * @param {any} request
 * @param {chrome.runtime.MessageSender} sender
 * @param {Responder} sendResponse
 * @returns {boolean | undefined}
 */
const onMessageListener = (request, sender, sendResponse) => {
  switch (request.action) {
    case 'saveRequest':
      return handleSaveRequests(request, sender, sendResponse);
    case 'ping':
      return handlePingRequests(request, sender, sendResponse);
    default:
      return undefined;
  }
};

chrome.runtime.onMessage.addListener(onMessageListener);

console.log('Noematic scrape handler installed');
