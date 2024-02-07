use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde_derive::{Deserialize, Serialize};

/// The version of the message format.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageVersion(semver::Version);

impl MessageVersion {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        MessageVersion(semver::Version::new(major, minor, patch))
    }

    pub fn parse(version: &str) -> Result<Self, semver::Error> {
        let version = semver::Version::parse(version)?;
        Ok(MessageVersion(version))
    }

    pub const EXPECTED: MessageVersion = MessageVersion::new(0, 1, 0);
}

impl From<semver::Version> for MessageVersion {
    fn from(version: semver::Version) -> Self {
        MessageVersion(version)
    }
}

/// Wrap a String in a newtype
macro_rules! wrap_string {
    ($name:ident) => {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct $name(String);

        impl $name {
            pub const fn new(value: String) -> Self {
                Self(value)
            }

            pub fn into_inner(self) -> String {
                self.0
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl ToSql for $name {
            fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
                self.0.to_sql()
            }
        }

        impl FromSql for $name {
            fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
                String::column_result(value).map(Self)
            }
        }
    };
}

wrap_string!(CorrelationId);
wrap_string!(Url);
wrap_string!(Title);
wrap_string!(InnerText);
wrap_string!(Query);

/// Messages that are sent from the client (extension) to the host.
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub version: MessageVersion,
    #[serde(flatten)]
    pub action: Action,
    #[serde(rename = "correlationId")]
    pub correlation_id: CorrelationId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum Action {
    #[serde(rename = "connectRequest")]
    ConnectRequest { payload: ConnectRequestPayload },
    #[serde(rename = "saveRequest")]
    SaveRequest { payload: SaveRequestPayload },
    #[serde(rename = "searchRequest")]
    SearchRequest { payload: SearchRequestPayload },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectRequestPayload {
    pub persist: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveRequestPayload {
    pub url: Url,
    pub title: Title,
    #[serde(rename = "innerText")]
    pub inner_text: InnerText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequestPayload {
    pub query: Query,
}

/// Messages that are sent from the host to the client (extension).
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub version: MessageVersion,
    #[serde(flatten)]
    pub action: ResponseAction,
    #[serde(rename = "correlationId")]
    pub correlation_id: CorrelationId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ResponseAction {
    #[serde(rename = "connectResponse")]
    ConnectResponse { payload: ConnectResponsePayload },
    #[serde(rename = "saveResponse")]
    SaveResponse { payload: SaveResponsePayload },
    #[serde(rename = "searchResponse")]
    SearchResponse { payload: SearchResponsePayload },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectResponsePayload {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveResponsePayload {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub url: Url,
    pub title: Title,
    #[serde(rename = "innerText")]
    pub inner_text: InnerText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponsePayload {
    pub query: Query,
    pub results: Vec<Site>,
}
