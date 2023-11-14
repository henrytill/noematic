mod db;
pub mod message;

use rusqlite::Connection;
use serde_json::Value;

use message::{
    Action, Request, Response, ResponseAction, SaveResponsePayload, SearchResponsePayload, Version,
};

#[derive(Debug)]
enum ErrorImpl {
    Sqlite(rusqlite::Error),
    Semver(semver::Error),
    MissingVersion,
    InvalidVersion,
}

#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorImpl>,
}

impl Error {
    fn new(inner: ErrorImpl) -> Self {
        let inner = Box::new(inner);
        Self { inner }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self.inner {
            ErrorImpl::Sqlite(ref e) => write!(f, "SQLite error: {}", e),
            ErrorImpl::Semver(ref e) => write!(f, "Semver error: {}", e),
            ErrorImpl::MissingVersion => write!(f, "Missing version"),
            ErrorImpl::InvalidVersion => write!(f, "Invalid version"),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(other: rusqlite::Error) -> Self {
        Self::new(ErrorImpl::Sqlite(other))
    }
}

impl From<semver::Error> for Error {
    fn from(other: semver::Error) -> Self {
        Self::new(ErrorImpl::Semver(other))
    }
}

impl From<db::Error> for Error {
    fn from(other: db::Error) -> Self {
        match other {
            db::Error::Sqlite(e) => Self::new(ErrorImpl::Sqlite(e)),
            db::Error::InvalidVersion => Self::new(ErrorImpl::InvalidVersion),
        }
    }
}

impl From<message::Error> for Error {
    fn from(other: message::Error) -> Self {
        match other {
            message::Error::Semver(e) => Self::new(ErrorImpl::Semver(e)),
        }
    }
}

impl std::error::Error for Error {}

pub struct Context {
    connection: rusqlite::Connection,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let mut connection = Connection::open_in_memory()?;
        db::init_tables(&mut connection)?;
        let context = Self { connection };
        Ok(context)
    }
}

pub fn handle_request(context: &mut Context, request: Request) -> Result<Response, Error> {
    let version = request.version;
    let correlation_id = request.correlation_id;

    match request.action {
        Action::SaveRequest { payload } => {
            db::upsert_site(&context.connection, payload)?;
            let payload = SaveResponsePayload {};
            let action = ResponseAction::SaveResponse { payload };
            let response = Response {
                version,
                action,
                correlation_id,
            };
            Ok(response)
        }
        Action::SearchRequest { payload } => {
            let results = db::search_sites(&context.connection, payload)?;
            let payload = SearchResponsePayload { results };
            let action = ResponseAction::SearchResponse { payload };
            let response = Response {
                version,
                action,
                correlation_id,
            };
            Ok(response)
        }
    }
}

/// Extracts the version from the message.
pub fn extract_version(value: &Value) -> Result<Version, Error> {
    let version = value["version"]
        .as_str()
        .ok_or(Error::new(ErrorImpl::MissingVersion))?;
    let version = Version::parse(version)?;
    Ok(version)
}
