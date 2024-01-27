mod db;
pub mod message;

use std::{fs, io, path::Path};

use directories::ProjectDirs;
use regex::Regex;
use serde_json::Value;

use message::{
    Action, ConnectResponsePayload, Query, Request, Response, ResponseAction, SaveResponsePayload,
    SearchResponsePayload, Version,
};

#[derive(Debug)]
enum ErrorImpl {
    Io(io::Error),
    Sqlite(rusqlite::Error),
    Semver(semver::Error),
    MissingHomeDir,
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
        match self.inner.as_ref() {
            ErrorImpl::Io(e) => write!(f, "IO error: {}", e),
            ErrorImpl::Sqlite(e) => write!(f, "SQLite error: {}", e),
            ErrorImpl::Semver(e) => write!(f, "Semver error: {}", e),
            ErrorImpl::MissingHomeDir => write!(f, "Missing home directory"),
            ErrorImpl::MissingVersion => write!(f, "Missing version"),
            ErrorImpl::InvalidVersion => write!(f, "Invalid version"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Self::new(ErrorImpl::Io(other))
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

#[derive(Debug)]
enum Connection {
    InMemory(rusqlite::Connection),
    Persistent(rusqlite::Connection),
}

impl Connection {
    fn inner(&self) -> &rusqlite::Connection {
        match self {
            Self::InMemory(connection) => connection,
            Self::Persistent(connection) => connection,
        }
    }

    fn inner_mut(&mut self) -> &mut rusqlite::Connection {
        match self {
            Self::InMemory(connection) => connection,
            Self::Persistent(connection) => connection,
        }
    }

    fn upgrade(&mut self, db_path: impl AsRef<Path>) -> Result<&mut rusqlite::Connection, Error> {
        if let Connection::Persistent(connection) = self {
            return Ok(connection);
        }
        let connection = rusqlite::Connection::open(db_path)?;
        let _prev = std::mem::replace(self, Self::Persistent(connection));
        let connection = self.inner_mut();
        Ok(connection)
    }
}

pub struct Context {
    connection: Connection,
    process: Box<dyn Fn(Query) -> String>,
}

fn make_process(re: Regex) -> impl Fn(Query) -> String {
    move |query| {
        let input = query.as_str();
        re.replace_all(input, " ").trim().to_string()
    }
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let mut connection = rusqlite::Connection::open_in_memory()?;
        db::init_tables(&mut connection)?;
        let connection = Connection::InMemory(connection);
        let process_regex = Regex::new(r"\W+").unwrap();
        let process = Box::new(make_process(process_regex));
        let context = Self {
            connection,
            process,
        };
        Ok(context)
    }
}

fn get_project_dirs() -> Result<ProjectDirs, Error> {
    ProjectDirs::from("com.github", "henrytill", "noematic")
        .ok_or(Error::new(ErrorImpl::MissingHomeDir))
}

pub fn handle_request(context: &mut Context, request: Request) -> Result<Response, Error> {
    let version = request.version;
    let correlation_id = request.correlation_id;

    let connection = context.connection.inner();

    match request.action {
        Action::ConnectRequest { payload } => {
            if payload.persist {
                let db_path = {
                    let project_dirs: ProjectDirs = get_project_dirs()?;
                    let db_dir = project_dirs.data_dir();
                    fs::create_dir_all(&db_dir)?;
                    db_dir.join("db.sqlite3")
                };
                let connection = context.connection.upgrade(db_path)?;
                db::init_tables(connection)?;
            }
            let response = {
                let payload = ConnectResponsePayload {};
                let action = ResponseAction::ConnectResponse { payload };
                Response {
                    version,
                    action,
                    correlation_id,
                }
            };
            Ok(response)
        }
        Action::SaveRequest { payload } => {
            db::upsert_site(connection, payload)?;
            let response = {
                let payload = SaveResponsePayload {};
                let action = ResponseAction::SaveResponse { payload };
                Response {
                    version,
                    action,
                    correlation_id,
                }
            };
            Ok(response)
        }
        Action::SearchRequest { payload } => {
            let process = context.process.as_ref();
            let query = payload.query.clone();
            let results = db::search_sites(connection, payload, process)?;
            let response = {
                let payload = SearchResponsePayload { query, results };
                let action = ResponseAction::SearchResponse { payload };
                Response {
                    version,
                    action,
                    correlation_id,
                }
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
