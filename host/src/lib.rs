#![warn(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]

mod db;
pub mod message;

use std::path::Path;

use anyhow::Error;
use regex::Regex;
use serde_json::Value;

use message::{
    MessageVersion, Query, RemoveResponsePayload, Request, RequestAction, Response, ResponseAction,
    SaveResponsePayload, SearchResponseHeaderPayload,
};

const FIELD_VERSION: &str = "version";
const MSG_MISSING_VERSION: &str = "Missing version";

#[derive(Debug)]
enum Connection {
    InMemory(rusqlite::Connection),
    Persistent(rusqlite::Connection),
}

impl AsRef<rusqlite::Connection> for Connection {
    fn as_ref(&self) -> &rusqlite::Connection {
        match self {
            Connection::InMemory(connection) | Connection::Persistent(connection) => connection,
        }
    }
}

pub struct Context {
    connection: Connection,
    process: Box<dyn Fn(&Query) -> String>,
}

fn make_process(re: Regex) -> impl Fn(&Query) -> String {
    move |query| {
        let input = query.as_str();
        re.replace_all(input, " ").trim().to_string()
    }
}

impl Context {
    const REGEX_WHITESPACE: &'static str = r"\W+";

    /// # Errors
    ///
    /// Returns an error if the database initialization or regex compilation fails.
    pub fn in_memory() -> Result<Context, Error> {
        let mut connection = rusqlite::Connection::open_in_memory()?;
        db::init_tables(&mut connection)?;
        let connection = Connection::InMemory(connection);
        let process_regex = Regex::new(Context::REGEX_WHITESPACE)?;
        let process = Box::new(make_process(process_regex));
        let context = Context {
            connection,
            process,
        };
        Ok(context)
    }

    /// # Errors
    ///
    /// Returns an error if the database cannot be opened, initialization fails, or regex compilation fails.
    pub fn persistent(db_path: impl AsRef<Path>) -> Result<Context, Error> {
        let mut connection = rusqlite::Connection::open(db_path.as_ref())?;
        db::init_tables(&mut connection)?;
        let connection = Connection::Persistent(connection);
        let process_regex = Regex::new(Context::REGEX_WHITESPACE)?;
        let process = Box::new(make_process(process_regex));
        let context = Context {
            connection,
            process,
        };
        Ok(context)
    }
}

/// # Errors
///
/// Returns an error if the database operations fail.
pub fn handle_request(context: &mut Context, request: Request) -> Result<Vec<Response>, Error> {
    let version = request.version;
    let correlation_id = request.correlation_id;

    let connection = context.connection.as_ref();

    match request.action {
        RequestAction::SaveRequest { payload } => {
            db::upsert_site(connection, &payload)?;
            let response = {
                let payload = SaveResponsePayload {};
                let action = ResponseAction::SaveResponse { payload };
                Response {
                    version,
                    action,
                    correlation_id,
                }
            };
            Ok(vec![response])
        }
        RequestAction::RemoveRequest { payload } => {
            db::remove(connection, &payload)?;
            let response = {
                let payload = RemoveResponsePayload {};
                let action = ResponseAction::RemoveResponse { payload };
                Response {
                    version,
                    action,
                    correlation_id,
                }
            };
            Ok(vec![response])
        }
        RequestAction::SearchRequest { payload } => {
            let process = context.process.as_ref();
            let query = payload.query.clone();
            let page_num = payload.page_num;
            let (results, has_more) = db::search_sites(connection, &payload, process)?;
            let header = {
                let page_length = results.len();
                let version = version.clone();
                let correlation_id = correlation_id.clone();
                let payload = SearchResponseHeaderPayload {
                    query,
                    page_num,
                    page_length,
                    has_more,
                };
                let action = ResponseAction::SearchResponseHeader { payload };
                Response {
                    version,
                    action,
                    correlation_id,
                }
            };
            let mut ret = vec![header];
            for payload in results {
                let version = version.clone();
                let correlation_id = correlation_id.clone();
                let action = ResponseAction::SearchResponseSite { payload };
                let response = Response {
                    version,
                    action,
                    correlation_id,
                };
                ret.push(response);
            }
            Ok(ret)
        }
    }
}

/// # Errors
///
/// Returns an error if the version field is missing or cannot be parsed.
pub fn extract_version(value: &Value) -> Result<MessageVersion, Error> {
    let version = value[FIELD_VERSION]
        .as_str()
        .ok_or_else(|| Error::msg(MSG_MISSING_VERSION))?;
    let version = MessageVersion::parse(version)?;
    Ok(version)
}
