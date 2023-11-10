use serde::{Deserialize, Serialize};

/// This is the JSON format of the messages that are sent to the host.
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub version: u64,
    #[serde(flatten)]
    pub action: Action,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum Action {
    #[serde(rename = "saveRequest")]
    SaveRequest { payload: SavePayload },
    #[serde(rename = "searchRequest")]
    SearchRequest { payload: SearchPayload },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SavePayload {
    pub title: String,
    #[serde(rename = "innerText")]
    pub inner_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchPayload {
    pub query: String,
}

/// This is the JSON format of the messages that are sent from the host.
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub version: u64,
    #[serde(flatten)]
    pub action: ResponseAction,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ResponseAction {
    #[serde(rename = "saveResponse")]
    SaveResponse { payload: SaveResponsePayload },
    #[serde(rename = "searchResponse")]
    SearchResponse { payload: SearchResponsePayload },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveResponsePayload {
    pub status: String,
    pub details: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponsePayload {
    pub results: Vec<String>,
}
