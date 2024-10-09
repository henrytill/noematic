import { SCHEMA_VERSION } from '../common/common.mjs';
import { SearchResult } from './search-result.mjs';

/**
 * Processes and displays the search results contained in a search response.
 *
 * @param {import('../common/types.js').Responses} responses
 * @returns {void}
 */
const handleSearchResponse = (responses) => {
  const resultsContainer = document.getElementById('results-container');
  if (!resultsContainer) {
    throw new Error('No main element found');
  }
  resultsContainer.innerHTML = '';
  if (responses.inner.length === 0) {
    resultsContainer.innerHTML = 'No results found';
    return;
  }
  for (const { action, payload } of responses.inner) {
    if (action !== 'searchResponseSite') {
      continue;
    }
    const { url, title, snippet } = payload;
    const resultElement = /** @type {SearchResult} */ (document.createElement('search-result'));
    resultElement.title = title;
    resultElement.href = url;
    resultElement.snippet = snippet;
    resultsContainer.appendChild(resultElement);
  }
};

/**
 * @param {string} value
 * @returns {void}
 */
const search = (value) => {
  if (value.length === 0) {
    return;
  }
  chrome.runtime
    .sendMessage({
      version: SCHEMA_VERSION,
      action: 'searchRequest',
      payload: { query: value, pageNum: 0, pageLength: 100 },
    })
    .then((response) => {
      handleSearchResponse(response);
      console.debug('response', response);
    });
};

/** @returns {void} */
const main = () => {
  const urlParams = new URLSearchParams(window.location.search);
  const query = urlParams.get('q');
  const searchInput = /** @type {HTMLFormElement?} */ (document.getElementById('search-input'));
  if (searchInput === null) {
    throw new Error('No search input found');
  }
  searchInput.value = query;
  if (query === null) return;
  search(query);
};

main();
