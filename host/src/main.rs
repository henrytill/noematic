use std::io::{self, BufReader, BufWriter, Read, Write};

use tempfile::NamedTempFile;

use noematic::message::{
    Action, Request, Response, ResponseAction, SaveResponsePayload, SearchResponsePayload,
};

#[derive(Debug)]
struct Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

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
) -> Result<(), Box<dyn std::error::Error>> {
    let request: Request = serde_json::from_slice(message)?;

    let correlation_id = request.correlation_id;

    let response = match request.action {
        Action::SaveRequest { payload: _ } => {
            let payload = SaveResponsePayload {
                status: "Success".to_string(),
                details: "Item saved".to_string(),
            };
            let action = ResponseAction::SaveResponse { payload };
            Response {
                action,
                correlation_id,
            }
        }
        Action::SearchRequest { payload: _ } => {
            let payload = SearchResponsePayload {
                results: vec!["Item1".to_string(), "Item2".to_string()],
            };
            let action = ResponseAction::SearchResponse { payload };
            Response {
                action,
                correlation_id,
            }
        }
    };

    let response_string = serde_json::to_string(&response)?;
    let response_length = (response_string.len() as u32).to_ne_bytes();
    writer.write_all(&response_length)?;
    writer.write_all(response_string.as_bytes())?;
    writer.flush()?;

    Ok(())
}

fn extract_version(message: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let value: serde_json::Value = serde_json::from_slice(message)?;

    let version = value["version"]
        .as_str()
        .ok_or_else(|| Error {})?
        .to_owned();

    Ok(version)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _temp_file = NamedTempFile::with_prefix("noematic-")?;
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());

    while let Some(length) = read_length(&mut reader)? {
        let message = read_message(&mut reader, length)?;

        let _version = extract_version(&message)?;

        if let Err(e) = handle_json_message(&mut writer, &message) {
            panic!("Error handling JSON message: {:?}", e);
        }
    }

    Ok(())
}
