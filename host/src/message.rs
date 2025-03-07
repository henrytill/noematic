//! The message format used to communicate between the host and the client (extension).
//!
//! See the [`Request`] and [`Response`] types for the message format.
//!
//! The message format is a simple JSON-based protocol.
//!
//! The message format is versioned. The version is a semantic version string. The version is
//! included in the message to allow for future changes to the message format.
//!
//! The message format is designed to be extensible. The `action` field is a string that indicates
//! the type of message. The `payload` field is an object that contains the data for the message.
//!
//! The `correlationId` field is a string that is used to correlate requests and responses. The
//! `correlationId` is included in the response to a request to allow the client to match the
//! response to the request.

use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};

/// The version of the message format.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MessageVersion(semver::Version);

impl MessageVersion {
    pub const fn new(major: u64, minor: u64, patch: u64) -> MessageVersion {
        MessageVersion(semver::Version::new(major, minor, patch))
    }

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

/// Wraps a [`String`] in a newtype
macro_rules! wrap_string {
    ($name:ident) => {
        /// A newtype that wraps a [`String`].
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

/// Messages that are sent from the client (extension) to the host.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub version: MessageVersion,
    #[serde(flatten)]
    pub action: Action,
    pub correlation_id: CorrelationId,
}

/// The actions that the client (extension) can send to the host.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum Action {
    SaveRequest { payload: SaveRequestPayload },
    RemoveRequest { payload: RemoveRequestPayload },
    SearchRequest { payload: SearchRequestPayload },
}

/// The payload for the `saveRequest` action.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SaveRequestPayload {
    pub url: Url,
    pub title: Title,
    pub inner_text: InnerText,
}

/// The payload for the `removeRequest` action.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoveRequestPayload {
    pub url: Url,
}

/// The payload for the `searchRequest` action.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequestPayload {
    pub query: Query,
    pub page_num: usize,
    pub page_length: usize,
}

/// Messages that are sent from the host to the client (extension).
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub version: MessageVersion,
    #[serde(flatten)]
    pub action: ResponseAction,
    pub correlation_id: CorrelationId,
}

/// The actions that the host can send to the client (extension).
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum ResponseAction {
    SaveResponse { payload: SaveResponsePayload },
    RemoveResponse { payload: RemoveResponsePayload },
    SearchResponseHeader { payload: SearchResponseHeaderPayload },
    SearchResponseSite { payload: SearchResponseSitePayload },
}

/// The payload for the `saveResponse` action.
#[derive(Serialize, Deserialize, Debug)]
pub struct SaveResponsePayload {}

/// The payload for the `saveResponse` action.
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
