const kStyle = `
.search-result {
  text-align: left;
  border-bottom: 1px solid #dfe1e5;
  padding: 10px 0;
}

.search-result:last-child {
  border-bottom: none;
}

.search-result a {
  display: block;
  color: #1a0dab;
  font-size: 18px;
  text-decoration: none;
  margin-bottom: 5px;
}

.search-result a:hover,
.search-result a:focus {
  text-decoration: underline;
}

.search-result p {
  font-size: 14px;
  color: #545454;
  margin: 0;
}
`;

/**
 * A custom element that displays a search result.
 *
 * @extends HTMLElement
 */
export class SearchResult extends HTMLElement {
  constructor() {
    super();
    const shadow = this.attachShadow({ mode: 'open' });

    const style = document.createElement('style');
    style.textContent = kStyle;
    shadow.appendChild(style);

    const container = document.createElement('div');
    container.className = 'search-result';

    const titleLink = document.createElement('a');
    titleLink.setAttribute('part', 'title');
    titleLink.target = '_blank';
    container.appendChild(titleLink);

    const textSnippet = document.createElement('p');
    textSnippet.setAttribute('part', 'snippet');
    const slot = document.createElement('slot');
    textSnippet.appendChild(slot);
    container.appendChild(textSnippet);

    shadow.appendChild(container);
  }

  static get observedAttributes() {
    return ['href', 'title'];
  }

  updateLink() {
    const shadowRoot = this.shadowRoot;
    if (shadowRoot === null) {
      console.error('No shadow root found');
      return;
    }

    const titleLink = shadowRoot.querySelector('a');
    if (titleLink === null) {
      console.error('No title link found');
      return;
    }

    const href = this.getAttribute('href');
    if (href !== null) {
      titleLink.href = href;
    }

    const title = this.getAttribute('title');
    if (title !== null) {
      titleLink.textContent = title;
    }
  }

  connectedCallback() {
    this.updateLink();
  }

  /**
   * @param {string} _name
   * @param {any} _oldValue
   * @param {any} _newValue
   */
  attributeChangedCallback(_name, _oldValue, _newValue) {
    this.updateLink();
  }
}

const init = () => {
  customElements.define('search-result', SearchResult);
};

init();
