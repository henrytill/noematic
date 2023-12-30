import * as common from '../common/common.mjs';
import { SCHEMA_VERSION } from '../common/common.mjs';
import { SearchResult } from './search-result.mjs';

/**
 * @typedef {import('../common/types.js').SearchResponse} SearchResponse
 */

/**
 * Finds the first occurrence of the first word of the query in the innerText
 * and returns the surrounding text.
 *
 * @param {string} text - The full text to search within.
 * @param {string} query - The search query.
 * @param {number} maxLength - The maximum length of the resulting text snippet.
 * @returns {string} - The text snippet with the query word highlighted.
 */
function createSnippet(text, query, maxLength) {
    // Normalize spaces and replace newlines with a space.
    text = text.replace(/\s+/g, ' ');

    if (query.length === 0) {
        throw new Error('Unexpected empty query');
    }

    const queryWord = query.split(' ')[0].toLowerCase();
    const matchIndex = text.toLowerCase().indexOf(queryWord);

    if (matchIndex === -1) {
        return common.abbreviate(text, maxLength);
    }

    // Calculate the start index for the snippet, ensuring it's within bounds and on a word boundary.
    let start = Math.max(0, matchIndex - Math.floor(maxLength / 2));
    start = text.lastIndexOf(' ', start - 1) + 1;

    // Calculate the end index based on the adjusted start, ensuring it's within bounds.
    let end = start + maxLength;
    end = end <= text.length ? text.indexOf(' ', end) : text.length;
    end = end === -1 ? text.length : end;

    // Prefix and suffix ellipses are added only if the snippet doesn't start or end at the text bounds.
    const prefix = start > 0 ? '...' : '';
    const suffix = end < text.length ? '...' : '';

    return prefix + text.substring(start, end) + suffix;
}

/**
 * Processes and displays the search results
 * @param {SearchResponse} response
 * @returns {void}
 */
function handleSearchResponse(response) {
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
    for (const { title, url, innerText } of results) {
        const resultElement = /** @type {SearchResult} */ (document.createElement('search-result'));
        resultElement.title = title;
        resultElement.href = url;
        resultElement.snippet = createSnippet(innerText, query, 200);
        resultsContainer.appendChild(resultElement);
    }
}

/**
 * @param {string} value
 * @returns {void}
 */
function search(value) {
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
}

/**
 * @returns {void}
 */
function main() {
    const urlParams = new URLSearchParams(window.location.search);
    const query = urlParams.get('q');
    const searchInput = /** @type {HTMLFormElement?} */ (document.getElementById('search-input'));
    if (searchInput === null) {
        throw new Error('No search input found');
    }
    searchInput.value = query;
    if (query === null) return;
    search(query);
}

main();
