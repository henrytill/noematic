use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MessageVersion(semver::Version);

impl MessageVersion {
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> MessageVersion {
        MessageVersion(semver::Version::new(major, minor, patch))
    }

    /// # Errors
    ///
    /// Returns an error if the version string cannot be parsed as a semantic version.
    pub fn parse(version: &str) -> Result<MessageVersion, semver::Error> {
        let version = semver::Version::parse(version)?;
        Ok(MessageVersion(version))
    }

    pub const EXPECTED: MessageVersion = MessageVersion::new(0, 1, 0);
}

impl From<semver::Version> for MessageVersion {
    fn from(version: semver::Version) -> MessageVersion {
        MessageVersion(version)
    }
}

macro_rules! wrap_string {
    ($name:ident) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
        pub struct $name(String);

        impl $name {
            pub const fn new(value: String) -> Self {
                Self(value)
            }

            pub fn into_inner(self) -> String {
                self.0
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
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
wrap_string!(Snippet);
wrap_string!(Query);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SaveRequestPayload {
    pub url: Url,
    pub title: Title,
    pub inner_text: InnerText,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoveRequestPayload {
    pub url: Url,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequestPayload {
    pub query: Query,
    pub page_num: usize,
    pub page_length: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum RequestAction {
    SaveRequest { payload: SaveRequestPayload },
    RemoveRequest { payload: RemoveRequestPayload },
    SearchRequest { payload: SearchRequestPayload },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub version: MessageVersion,
    #[serde(flatten)]
    pub action: RequestAction,
    pub correlation_id: CorrelationId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveResponsePayload {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveResponsePayload {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponseHeaderPayload {
    pub query: Query,
    pub page_num: usize,
    pub page_length: usize,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponseSitePayload {
    pub url: Url,
    pub title: Title,
    pub snippet: Snippet,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum ResponseAction {
    SaveResponse {
        payload: SaveResponsePayload,
    },
    RemoveResponse {
        payload: RemoveResponsePayload,
    },
    SearchResponseHeader {
        payload: SearchResponseHeaderPayload,
    },
    SearchResponseSite {
        payload: SearchResponseSitePayload,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub version: MessageVersion,
    #[serde(flatten)]
    pub action: ResponseAction,
    pub correlation_id: CorrelationId,
}
