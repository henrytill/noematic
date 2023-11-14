use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde_derive::{Deserialize, Serialize};

pub enum Error {
    Semver(semver::Error),
}

impl From<semver::Error> for Error {
    fn from(other: semver::Error) -> Self {
        Error::Semver(other)
    }
}

/// The version of the message format.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(semver::Version);

impl Version {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Version(semver::Version::new(major, minor, patch))
    }

    pub fn parse(version: &str) -> Result<Self, Error> {
        let version = semver::Version::parse(version)?;
        Ok(Version(version))
    }
}

impl From<semver::Version> for Version {
    fn from(version: semver::Version) -> Self {
        Version(version)
    }
}

/// Wrap a String in a newtype
macro_rules! wrap_string {
    ($name:ident) => {
        #[derive(Serialize, Deserialize, Debug)]
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
    pub url: Url,
    pub title: Title,
    #[serde(rename = "innerText")]
    pub inner_text: InnerText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchPayload {
    pub query: Query,
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
    pub results: Vec<Site>,
}
