import { kSchemaVersion } from '../common/common.mjs';

/** @type  {EventListener} */
const listener = (event) => {
  event.preventDefault();
  const query = /** @type {HTMLInputElement?} */ (document.getElementById('search-input'));
  if (query == undefined) {
    throw new Error('No search input found');
  }
  const { value } = query;
  if (value.length === 0) {
    return;
  }
  chrome.runtime
    .sendMessage({ version: kSchemaVersion, action: 'searchRequest', payload: { query: value } })
    .then((response) => console.log('response', response));
};

/**
 * @returns {void}
 */
const main = () => {
  const searchForm = document.getElementById('search-form');
  if (!searchForm) {
    throw new Error('No search form found');
  }
  searchForm.addEventListener('submit', listener);
};

main();
