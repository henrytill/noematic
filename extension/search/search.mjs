const attachSearchListener = (listener) => {
  const searchForm = document.getElementById('search-form');
  if (!searchForm) {
    throw new Error('No search form found');
  }
  searchForm.addEventListener('submit', listener);
};

const sendQuery = async (query) => {
  let message = { action: 'searchRequest', payload: { query: query } };
  let response = await chrome.runtime.sendMessage(message);
  console.log('response', response);
};

const listener = (event) => {
  event.preventDefault();
  const query = document.getElementById('search-input');
  if (!query) {
    throw new Error('No search input found');
  }
  const queryValue = query.value;
  if (queryValue.length === 0) {
    return;
  }
  sendQuery(queryValue);
};

const main = () => {
  attachSearchListener(listener);
};

main();
