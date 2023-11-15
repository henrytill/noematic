/**
 * Defines the structure of a search result object.
 * @typedef {Object} SearchResultData
 * @property {string} title - The title of the search result.
 * @property {string} url - The URL of the search result.
 * @property {string} snippet - A snippet of text from the search result.
 */

/**
 * A custom element that displays a search result.
 *
 * @extends HTMLElement
 */
export class SearchResult extends HTMLElement {
  constructor() {
    super();
    const shadow = this.attachShadow({ mode: 'open' });

    /** @type {HTMLLinkElement} */
    const linkElem = document.createElement('link');
    linkElem.setAttribute('rel', 'stylesheet');
    linkElem.setAttribute('href', 'search-result.css');
    shadow.appendChild(linkElem);

    /** @type {HTMLDivElement} */
    const container = document.createElement('div');
    container.className = 'search-result';

    /** @type {HTMLAnchorElement} */
    const titleLink = document.createElement('a');
    titleLink.setAttribute('part', 'title'); // Expose this part for styling if necessary.
    titleLink.target = '_blank';

    /** @type {HTMLParagraphElement} */
    const textSnippet = document.createElement('p');
    textSnippet.setAttribute('part', 'snippet'); // Expose this part for styling if necessary.

    container.appendChild(titleLink);
    container.appendChild(textSnippet);

    shadow.appendChild(container);
  }

  /**
   * Sets the search result data.
   *
   * @param {SearchResultData} result - The search result data to display.
   */
  set result({ url, title, snippet }) {
    const shadowRoot = this.shadowRoot;
    if (shadowRoot === null) {
      throw new Error('Shadow root not found.');
    }

    const titleLink = shadowRoot.querySelector('a');
    const textSnippet = shadowRoot.querySelector('p');
    if (titleLink === null || textSnippet === null) {
      throw new Error('Could not find elements in shadow root.');
    }

    titleLink.href = url;
    titleLink.textContent = title;
    textSnippet.textContent = snippet;
  }
}

export default () => {
  customElements.define('search-result', SearchResult);
};
