use std::io::{self, BufReader, BufWriter, Read, Write};

use serde_derive::{Deserialize, Serialize};
use tempfile::NamedTempFile;

#[derive(Debug)]
struct Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

/// This is the JSON format of the messages that are sent to the host.
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    #[serde(flatten)]
    action: Action,
    #[serde(rename = "correlationId")]
    correlation_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
enum Action {
    #[serde(rename = "saveRequest")]
    SaveRequest { payload: SavePayload },
    #[serde(rename = "searchRequest")]
    SearchRequest { payload: SearchPayload },
}

#[derive(Serialize, Deserialize, Debug)]
struct SavePayload {
    title: String,
    #[serde(rename = "innerText")]
    inner_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchPayload {
    query: Query,
}

#[derive(Serialize, Deserialize, Debug)]
struct Query {
    query: String,
}

/// This is the JSON format of the messages that are sent from the host.
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    #[serde(flatten)]
    action: ResponseAction,
    #[serde(rename = "correlationId")]
    correlation_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
enum ResponseAction {
    #[serde(rename = "saveResponse")]
    SaveResponse { payload: SaveResponsePayload },
    #[serde(rename = "searchResponse")]
    SearchResponse { payload: SearchResponsePayload },
}

#[derive(Serialize, Deserialize, Debug)]
struct SaveResponsePayload {
    status: String,
    details: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResponsePayload {
    results: Vec<String>,
}

/// Reads the length of the message from the reader.
fn read_length(reader: &mut impl Read) -> io::Result<Option<u32>> {
    let mut bytes = [0; 4];
    match reader.read_exact(&mut bytes) {
        Ok(_) => Ok(Some(u32::from_ne_bytes(bytes))),
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(e),
    }
}

/// Reads the message of the given length from the reader.
fn read_message(reader: &mut impl Read, length: u32) -> io::Result<Vec<u8>> {
    let length = length as usize;
    let mut message = vec![0; length];
    reader.read_exact(&mut message)?;
    Ok(message)
}

fn handle_json_message(
    writer: &mut impl Write,
    message: &[u8],
    correlation_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_message: Request = serde_json::from_slice(message)?;

    let response = match json_message.action {
        Action::SaveRequest { payload: _ } => Response {
            action: ResponseAction::SaveResponse {
                payload: SaveResponsePayload {
                    status: "Success".to_string(),
                    details: "Item saved".to_string(),
                },
            },
            correlation_id: correlation_id.to_owned(),
        },
        Action::SearchRequest { payload: _ } => Response {
            action: ResponseAction::SearchResponse {
                payload: SearchResponsePayload {
                    results: vec!["Item1".to_string(), "Item2".to_string()],
                },
            },
            correlation_id: correlation_id.to_owned(),
        },
    };

    let response_string = serde_json::to_string(&response)?;
    let response_length = (response_string.len() as u32).to_ne_bytes();
    writer.write_all(&response_length)?;
    writer.write_all(response_string.as_bytes())?;
    writer.flush()?;

    Ok(())
}

fn extract_correlation_id(message: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let value: serde_json::Value = serde_json::from_slice(message)?;

    let correlation_id = value["correlationId"]
        .as_str()
        .ok_or_else(|| Error {})?
        .to_owned();

    Ok(correlation_id)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _temp_file = NamedTempFile::with_prefix("noematic-")?;
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());

    while let Some(length) = read_length(&mut reader)? {
        let message = read_message(&mut reader, length)?;

        let correlation_id = extract_correlation_id(&message)?;
        if let Err(e) = handle_json_message(&mut writer, &message, &correlation_id) {
            eprintln!("Error handling JSON message: {:?}", e);
        }
    }

    Ok(())
}
