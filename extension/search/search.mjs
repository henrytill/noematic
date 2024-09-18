import * as common from '../common/common.mjs';
import { SCHEMA_VERSION } from '../common/common.mjs';
import { SearchResult } from './search-result.mjs';

/**
 * @typedef {import('../common/types.js').SearchResponse} SearchResponse
 */

/**
 * Processes and displays the search results contained in a search response.
 *
 * @param {SearchResponse} response
 * @returns {void}
 */
const handleSearchResponse = (response) => {
  const resultsContainer = document.getElementById('results-container');
  if (!resultsContainer) {
    throw new Error('No main element found');
  }
  resultsContainer.innerHTML = '';
  const {
    payload: { query, results },
  } = response;
  if (results.length === 0) {
    resultsContainer.innerHTML = 'No results found';
    return;
  }
  for (const { title, url, snippet } of results) {
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
      payload: { query: value },
    })
    .then((response) => {
      handleSearchResponse(response);
      console.log('response', response);
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
