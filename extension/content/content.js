const scrapePage = () => {
  const title = document.title;
  const innerText = document.body.innerText;
  return { title, innerText };
};

const handleSaveRequests = (request, _sender, sendResponse) => {
  request.payload = scrapePage();
  chrome.runtime.sendMessage(request).then((response) => {
    response.action = 'saveResponse';
    sendResponse(response);
  });
  return true;
};

const handlePingRequests = (_request, _sender, sendResponse) => {
  sendResponse({ action: 'pong' });
  return true;
};

const listener = (request, sender, sendResponse) => {
  switch (request.action) {
    case 'saveRequest':
      return handleSaveRequests(request, sender, sendResponse);
    case 'ping':
      return handlePingRequests(request, sender, sendResponse);
    default:
      throw new Error('Unknown action');
  }
};

const main = () => {
  chrome.runtime.onMessage.addListener(listener);
  console.log('Noematic scrape handler installed');
};

main();
