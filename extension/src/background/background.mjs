// @ts-ignore
import init, { execute } from '../generated/noematic_web.js';

/**
 * @typedef {import('../common/common.ts').UUID} UUID
 */

/**
 * @param {any} request
 * @param {chrome.runtime.MessageSender} _sender
 * @param {Responder} sendResponse
 * @returns {void}
 */
const messageListener = (request, _sender, sendResponse) => {
  /** @type {UUID} */
  const correlationId = crypto.randomUUID();
  request.correlationId = correlationId;
  console.log('request', request);
  const response = execute(request);
  sendResponse(response);
  return;
};

/**
 * @returns {Promise<void>}
 */
const main = async () => {
  await init();
  chrome.runtime.onMessage.addListener(messageListener);
};

main();
