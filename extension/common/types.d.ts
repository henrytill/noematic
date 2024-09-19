export type UUID = `${string}-${string}-${string}-${string}-${string}`;

export type Responder = (response?: any) => void;

export type State = {
  url: URL | null;
  tab: chrome.tabs.Tab;
};

export type SaveResponse = {
  version: string;
  action: 'saveResponse';
  payload: null;
  correlationId: UUID;
};

export type SearchResponseHeaderPayload = {
  query: string;
  pageNum: number;
  pageLength: number;
  hasMore: boolean;
};

export type SearchResponseHeader = {
  version: string;
  action: 'searchResponseHeader';
  payload: SearchResponseHeaderPayload;
  correlationId: UUID;
};

export type SearchResponseSitePayload = {
  url: string;
  title: string;
  snippet: string;
};

export type SearchResponseSite = {
  version: string;
  action: 'searchResponseSite';
  payload: SearchResponseSitePayload;
  correlationId: UUID;
};

export type Response = SaveResponse | SearchResponseHeader | SearchResponseSite;

export type Responses = {
  inner: Response[];
};
