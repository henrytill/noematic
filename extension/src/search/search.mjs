import { kSchemaVersion } from '../common/common.mjs';

/**
 * @typedef {import('../common/types.js').SearchResponse} SearchResponse
 */

/**
 * Abbreviates text to a specified maximum length.
 * @param {string} text - The text to abbreviate.
 * @param {number} maxLength - The maximum length of the abbreviated text.
 * @returns {string} - The abbreviated text.
 */
const abbreviateText = (text, maxLength) => {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength - 3) + '...';
};

/**
 * Finds the first occurrence of the first word of the query in the innerText
 * and returns the surrounding text.
 *
 * @param {string} text - The full text to search within.
 * @param {string} query - The search query.
 * @param {number} maxLength - The maximum length of the resulting text snippet.
 * @returns {string} - The text snippet with the query word highlighted.
 */
const highlightQueryInText = (text, query, maxLength) => {
  // Normalize spaces and replace newlines with a space.
  text = text.replace(/\s+/g, ' ');

  if (query.length === 0) {
    throw new Error('Unexpected empty query');
  }

  // Find the first word of the query in the text.
  const queryWord = query.split(' ')[0].toLowerCase();
  const matchIndex = text.toLowerCase().indexOf(queryWord);

  if (matchIndex === -1) {
    // If there's no match, return the abbreviated text.
    return abbreviateText(text, maxLength);
  }

  const halfMaxLength = Math.floor(maxLength / 2);

  // Calculate start and end indices for the snippet.
  let start = Math.max(0, matchIndex - halfMaxLength);
  let end = Math.min(text.length, matchIndex + queryWord.length + halfMaxLength);

  // Adjust start and end indices to word boundaries.
  start = text.lastIndexOf(' ', start) + 1 || start;
  end = text.indexOf(' ', end) !== -1 ? text.indexOf(' ', end) : end;

  // Build and return the snippet with ellipses if necessary.
  const prefix = start > 0 ? '...' : '';
  const suffix = end < text.length ? '...' : '';

  return prefix + text.substring(start, end) + suffix;
};

/**
 * Processes and displays the search results
 * @param {SearchResponse} response
 * @returns {void}
 */
const handleSearchResponse = (response) => {
  const resultsContainer = document.getElementById('results-container');
  if (!resultsContainer) {
    throw new Error('No main element found');
  }
  resultsContainer.innerHTML = '';
  const results = response.payload.results;
  if (results.length === 0) {
    resultsContainer.innerHTML = 'No results found';
    return;
  }

  const query = response.payload.query;

  for (const result of results) {
    const resultElement = document.createElement('div');
    resultElement.className = 'search-result';

    const linkElement = document.createElement('a');
    linkElement.href = result.url;
    linkElement.textContent = result.title;
    linkElement.target = '_blank';
    resultElement.appendChild(linkElement);

    const textElement = document.createElement('p');
    textElement.innerText = highlightQueryInText(result.innerText, query, 200);
    resultElement.appendChild(textElement);

    resultsContainer.appendChild(resultElement);
  }
};

/**
 * @param {string} value
 */
const search = (value) => {
  if (value.length === 0) {
    return;
  }
  chrome.runtime
    .sendMessage({ version: kSchemaVersion, action: 'searchRequest', payload: { query: value } })
    .then((response) => {
      handleSearchResponse(response);
      console.log('response', response);
    });
};

/**
 * @returns {void}
 */
const main = () => {
  const urlParams = new URLSearchParams(window.location.search);
  const query = urlParams.get('q');

  const searchInput = /** @type {HTMLFormElement?} */ (document.getElementById('search-input'));
  if (searchInput === null) {
    throw new Error('No search input found');
  }
  searchInput.value = query;

  if (query !== null) {
    search(query);
  }
};

main();
