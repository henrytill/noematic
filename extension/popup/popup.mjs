/**
 * @returns {void}
 */
const handleSearch = () => {
  chrome.tabs.create({ url: '/search/index.html' });
  window.close();
};

/**
 * @returns {void}
 */
const main = () => {
  document.getElementById('search')?.addEventListener('click', handleSearch);
};

main();
