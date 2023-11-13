use serde_derive::{Deserialize, Serialize};

/// The version of the message format.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u64);

impl Version {
    pub const fn new(version: u64) -> Self {
        Version(version)
    }
}

impl From<u64> for Version {
    fn from(version: u64) -> Self {
        Version(version)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CorrelationId(String);

impl CorrelationId {
    pub const fn new(correlation_id: String) -> Self {
        CorrelationId(correlation_id)
    }
}

impl From<String> for CorrelationId {
    fn from(correlation_id: String) -> Self {
        CorrelationId(correlation_id)
    }
}

/// Messages that are sent from the client (extension) to the host.
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub version: Version,
    #[serde(flatten)]
    pub action: Action,
    #[serde(rename = "correlationId")]
    pub correlation_id: CorrelationId,
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

/// Messages that are sent from the host to the client (extension).
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub version: Version,
    #[serde(flatten)]
    pub action: ResponseAction,
    #[serde(rename = "correlationId")]
    pub correlation_id: CorrelationId,
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
