//!
//! The library for the Noematic application.
//!
//! The library contains the core functionality of the application, including handling requests and
//! responses, and managing the database.
//!

mod db;
pub mod message;

use std::path::Path;

use anyhow::Error;
use regex::Regex;
use serde_json::Value;

use message::{
    Action, MessageVersion, Query, Request, Response, ResponseAction, SaveResponsePayload,
    SearchResponsePayload,
};

const MSG_MISSING_MESSAGE_VERSION: &str = "Missing message version";

#[derive(Debug)]
enum Connection {
    InMemory(rusqlite::Connection),
    Persistent(rusqlite::Connection),
}

impl AsRef<rusqlite::Connection> for Connection {
    fn as_ref(&self) -> &rusqlite::Connection {
        match self {
            Connection::InMemory(connection) => connection,
            Connection::Persistent(connection) => connection,
        }
    }
}

///
/// The context of the application.
///
/// The context contains the state of the application and is used to handle requests.
///
/// # Examples
///
/// ```
/// use noematic::Context;
///
/// let context = Context::in_memory().unwrap();
/// ```
///
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
    pub fn in_memory() -> Result<Context, Error> {
        let mut connection = rusqlite::Connection::open_in_memory()?;
        db::init_tables(&mut connection)?;
        let connection = Connection::InMemory(connection);
        let process_regex = Regex::new(r"\W+").unwrap();
        let process = Box::new(make_process(process_regex));
        let context = Context {
            connection,
            process,
        };
        Ok(context)
    }

    pub fn persistent(db_path: impl AsRef<Path>) -> Result<Context, Error> {
        let mut connection = rusqlite::Connection::open(db_path.as_ref())?;
        db::init_tables(&mut connection)?;
        let connection = Connection::Persistent(connection);
        let process_regex = Regex::new(r"\W+").unwrap();
        let process = Box::new(make_process(process_regex));
        let context = Context {
            connection,
            process,
        };
        Ok(context)
    }
}

///
/// Handles a request and returns a response.
///
/// # Arguments
///
/// * `context` - The context of the application.
/// * `request` - The request to handle.
///
/// # Returns
///
/// A response to the request.
///
/// # Errors
///
/// If an error occurs while handling the request.
///
/// # Example
///
/// ```
/// use noematic::Context;
/// use noematic::message::{Action, CorrelationId, MessageVersion, Query, Request, SearchRequestPayload};
///
/// let mut context = Context::in_memory().unwrap();
/// let request = {
///     let version = MessageVersion::parse("0.1.0").unwrap();
///     let action = {
///         let query = Query::new(String::from("hello"));
///         let payload = SearchRequestPayload { query };
///         Action::SearchRequest { payload }
///     };
///     let correlation_id = CorrelationId::new(String::from("218ecc9f-a91a-4b55-8b50-2b6672daa9a5"));
///     Request { version, action, correlation_id }
/// };
/// let response = noematic::handle_request(&mut context, request).unwrap();
/// ```
///
pub fn handle_request(context: &mut Context, request: Request) -> Result<Response, Error> {
    let version = request.version;
    let correlation_id = request.correlation_id;

    let connection = context.connection.as_ref();

    match request.action {
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

///
/// Extracts the message version from the message.
///
/// # Arguments
///
/// * `value` - The parsed message to extract the message version from.
///
/// # Returns
///
/// The version of the message.
///
/// # Errors
///
/// * If the message does not contain a version.
/// * If the version string does not parse to a valid [`MessageVersion`].
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use noematic::message::MessageVersion;
///
/// let request = json!({
///     "version": "0.1.0",
///     "action": "saveRequest",
///     "payload": {
///         "url": "https://en.wikipedia.org/wiki/Foobar",
///         "title": "Foobar - Wikipedia",
///         "innerText": "..."
///     },
///     "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
/// });
/// let version = noematic::extract_version(&request).unwrap();
/// assert_eq!(version, MessageVersion::new(0, 1, 0));
/// ```
///
pub fn extract_version(value: &Value) -> Result<MessageVersion, Error> {
    let version = value["version"]
        .as_str()
        .ok_or_else(|| Error::msg(MSG_MISSING_MESSAGE_VERSION))?;
    let version = MessageVersion::parse(version)?;
    Ok(version)
}
