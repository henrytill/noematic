use std::io::{self, BufReader, BufWriter, Read, Write};
use std::sync::mpsc;
use std::thread;

use serde_json::Value;

use noematic::message::{Request, Response, Version};

const EXPECTED_VERSION: Version = Version::new(1);

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    EndOfStream,
    MissingVersion,
    UnsupportedVersion,
    UnsupportedLength,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::EndOfStream => write!(f, "End of stream"),
            Error::MissingVersion => write!(f, "Missing version"),
            Error::UnsupportedVersion => write!(f, "Unsupported version"),
            Error::UnsupportedLength => write!(f, "Unsupported length"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl From<serde_json::Error> for Error {
    fn from(other: serde_json::Error) -> Self {
        Error::Json(other)
    }
}

impl std::error::Error for Error {}

/// Reads the length prefix of a message from the reader.
fn read_length(reader: &mut impl Read) -> io::Result<Option<u32>> {
    let mut bytes = [0; 4];
    match reader.read_exact(&mut bytes) {
        Ok(_) => Ok(Some(u32::from_ne_bytes(bytes))),
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(e),
    }
}

/// Reads a message of the given length from the reader.
fn read_message(reader: &mut impl Read, length: u32) -> io::Result<Vec<u8>> {
    let length = length as usize;
    let mut message = vec![0; length];
    reader.read_exact(&mut message)?;
    Ok(message)
}

/// Reads a message from the reader.
fn read(reader: &mut impl Read) -> Result<Vec<u8>, Error> {
    let maybe_length = read_length(reader)?;
    let length = maybe_length.ok_or(Error::EndOfStream)?;
    read_message(reader, length).map_err(Into::into)
}

/// Extracts the version from the message.
fn extract_version(value: &Value) -> Result<Version, Error> {
    let version = value["version"]
        .as_u64()
        .ok_or(Error::MissingVersion)?
        .to_owned();
    Ok(Version::new(version))
}

/// Writes the response to the writer.
fn write_response(writer: &mut impl Write, response: Response) -> Result<(), Error> {
    let response_bytes = serde_json::to_string(&response)?.into_bytes();
    let response_length = TryInto::<u32>::try_into(response_bytes.len())
        .or(Err(Error::UnsupportedLength))?
        .to_ne_bytes();
    writer.write_all(&response_length)?;
    writer.write_all(&response_bytes)?;
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    let (tx, rx) = mpsc::channel::<Result<Vec<u8>, Error>>();

    thread::spawn(move || loop {
        let result: Result<Vec<u8>, Error> = read(&mut reader);
        match tx.send(result) {
            Ok(_) => {}
            Err(_) => return,
        }
    });

    loop {
        let message = match rx.recv()? {
            Ok(message) => message,
            Err(Error::EndOfStream) => {
                break;
            }
            Err(e) => return Err(e.into()),
        };

        let json: Value = serde_json::from_slice(&message)?;

        let version = extract_version(&json)?;
        if version != EXPECTED_VERSION {
            return Err(Error::UnsupportedVersion.into());
        }

        let request: Request = serde_json::from_value(json)?;
        let response = noematic::handle_request(request);
        write_response(&mut writer, response)?;
    }

    Ok(())
}
