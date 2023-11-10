export type UUID = `${string}-${string}-${string}-${string}-${string}`;

export type Responder = (response: any) => void;

export type ResponderMap = Map<UUID, Responder>;

export type State = {
  url: URL;
  tab: chrome.tabs.Tab;
};
