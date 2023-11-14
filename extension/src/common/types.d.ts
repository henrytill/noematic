export type UUID = `${string}-${string}-${string}-${string}-${string}`;

export type Responder = (response: any) => void;

export type ResponderMap = Map<UUID, Responder>;

export type State = {
  url: URL | null;
  tab: chrome.tabs.Tab;
};

export type SearchResponsePayload = {
  query: string;
  results: Array<{
    url: string;
    title: string;
    innerText: string;
  }>;
};

export type SearchResponse = {
  version: string;
  action: 'searchResponse';
  payload: SearchResponsePayload;
  correlationId: UUID;
};
