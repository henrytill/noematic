/**
 * @typedef {import("./types.js").UUID} UUID
 * @typedef {import("./types.js").Responder} Responder
 * @typedef {import("./types.js").Response} Response
 * @typedef {import("./types.js").Responses} Responses
 */

export class MessageCollector {
  /** @type {UUID} */
  correlationId;
  /** @type {Responder} */
  responder;
  /** @type {Response[]} */
  responses = [];

  /**
   * @param {UUID} correlationId
   * @param {Responder} responder
   */
  constructor(correlationId, responder) {
    this.correlationId = correlationId;
    this.responder = responder;
  }

  /**
   * @param {Response} response
   * @returns {boolean}
   */
  push(response) {
    this.responses.push(response);
    console.debug('response', response);
    return this.collect();
  }

  /**
   * @returns {boolean}
   */
  collect() {
    const head = this.responses[0];
    if (head === undefined) {
      throw new Error('collect: top was undefined');
    }
    switch (head.action) {
      case 'saveResponse': {
        this.responder(head);
        return true;
      }
      case 'removeResponse': {
        this.responder(head);
        return true;
      }
      case 'searchResponseHeader': {
        const pageLength = head.payload.pageLength;
        if (pageLength !== this.responses.length - 1) {
          return false;
        }
        /** @type {Responses} */
        const responses = { inner: this.responses.slice(1) };
        this.responder(responses);
        this.responses = [];
        return true;
      }
      case 'searchResponseSite': {
        throw new Error('collect: top was searchResponseSite');
      }
    }
  }
}
