/**
 * @param {{query: string}} query
 */
const sendQuery = async (query) => {
  let message = { action: 'searchRequest', payload: { query: query } };
  let response = await chrome.runtime.sendMessage(message);
  console.log('response', response);
};

/** @type  {EventListener} */
const listener = (event) => {
  event.preventDefault();
  const query = /** @type {HTMLInputElement?} */ (document.getElementById('search-input'));
  if (!query) {
    throw new Error('No search input found');
  }
  const { value } = query;
  if (value.length === 0) {
    return;
  }
  sendQuery({ query: value });
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
