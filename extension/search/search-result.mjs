const template = document.createElement('template');

template.innerHTML = `
<style>
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
</style>
<div class="search-result">
  <a target="_blank"></a>
  <p><slot></slot></p>
</div>
`;

/**
 * A custom element that displays a search result.
 *
 * @extends HTMLElement
 */
export class SearchResult extends HTMLElement {
  static get observedAttributes() {
    return ['title', 'href'];
  }

  constructor() {
    super();
    const shadow = this.attachShadow({ mode: 'open' });
    shadow.appendChild(template.content.cloneNode(true));
  }

  connectedCallback() {}

  get title() {
    return this.getAttribute('title') ?? '';
  }

  set title(value) {
    // @ts-ignore
    super.title = value;
    this.setAttribute('title', value);
  }

  get href() {
    return this.getAttribute('href') ?? '';
  }

  set href(value) {
    this.setAttribute('href', value);
  }

  get snippet() {
    return this.innerHTML;
  }

  set snippet(value) {
    this.innerHTML = value;
  }

  /**
   * @param {string} name
   * @param {any} _oldValue
   * @param {any} newValue
   */
  attributeChangedCallback(name, _oldValue, newValue) {
    const shadow = this.shadowRoot;
    if (shadow === null) {
      return;
    }

    const titleLink = shadow.querySelector('a');
    if (titleLink === null) {
      return;
    }

    switch (name) {
      case 'title':
        titleLink.textContent = newValue;
        break;
      case 'href':
        titleLink.href = newValue;
        break;
      default:
        break;
    }
  }
}

/** @returns {void} */
const init = () => {
  customElements.define('search-result', SearchResult);
};

init();
