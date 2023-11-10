const nativeMessagingHost = 'com.github.henrytill.noematic';

const handleHostMessage = (responseMap, message) => {
  const correlationId = message.correlationId;
  const response = responseMap.get(correlationId);
  if (response) {
    responseMap.delete(correlationId);
    response(message);
  }
};

const handleHostDisconnect = (_) => {
  console.log('Disconnected from native messaging host');
};

const connectHost = (responseMap) => {
  const port = chrome.runtime.connectNative(nativeMessagingHost);
  port.onMessage.addListener(handleHostMessage.bind(null, responseMap));
  port.onDisconnect.addListener(handleHostDisconnect);
  return port;
};

const messageListener = (responseMap, hostPort, request, _sender, sendResponse) => {
  const correlationId = crypto.randomUUID();
  request.correlationId = correlationId;
  console.log('request', request);
  responseMap.set(correlationId, sendResponse);
  hostPort.postMessage(request);
  return true;
};

const main = () => {
  const responseMap = new Map();
  const hostPort = connectHost(responseMap);
  chrome.runtime.onMessage.addListener(messageListener.bind(null, responseMap, hostPort));
  console.log('Noematic background handler installed');
};

main();
