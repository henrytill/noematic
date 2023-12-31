use std::io::{self, BufReader, BufWriter, Read, Write};

use serde_json::Value;

use noematic::{
    message::{Request, Response, Version},
    Context,
};

const EXPECTED_VERSION: Version = Version::new(0, 1, 0);

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    Noematic(noematic::Error),
    UnsupportedVersion,
    UnsupportedLength,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::Noematic(e) => write!(f, "{}", e),
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

impl From<noematic::Error> for Error {
    fn from(other: noematic::Error) -> Self {
        Error::Noematic(other)
    }
}

impl std::error::Error for Error {}

/// Reads the length prefix of a message.
///
/// Returns `None` if the reader is at EOF.
fn read_length(reader: &mut impl Read) -> io::Result<Option<u32>> {
    let mut bytes = [0; 4];
    match reader.read_exact(&mut bytes) {
        Ok(_) => Ok(Some(u32::from_ne_bytes(bytes))),
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(e),
    }
}

/// Reads a message of the given length.
fn read_message(reader: &mut impl Read, length: u32) -> io::Result<Vec<u8>> {
    let length = length as usize;
    let mut message = vec![0; length];
    reader.read_exact(&mut message)?;
    Ok(message)
}

/// Reads a length `n`-prefixed bytestring into a vector of length `n`.
///
/// Returns `None` if the reader is at EOF.
fn read(reader: &mut impl Read) -> Result<Option<Vec<u8>>, Error> {
    let length = match read_length(reader)? {
        None => return Ok(None),
        Some(length) => length,
    };
    read_message(reader, length).map(Some).map_err(Into::into)
}

/// Serializes a response, prefixed by its serialized length, to the writer.
fn write_response(writer: &mut impl Write, response: Response) -> Result<(), Error> {
    let response_bytes = serde_json::to_string(&response)?.into_bytes();
    let response_length = u32::try_from(response_bytes.len())
        .or(Err(Error::UnsupportedLength))?
        .to_ne_bytes();
    writer.write_all(&response_length)?;
    writer.write_all(&response_bytes)?;
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());

    let mut context = Context::new()?;

    while let Some(message) = read(&mut reader)? {
        let json: Value = serde_json::from_slice(&message)?;

        let version = noematic::extract_version(&json)?;
        if version != EXPECTED_VERSION {
            return Err(Error::UnsupportedVersion);
        }

        let request: Request = serde_json::from_value(json)?;
        let response = noematic::handle_request(&mut context, request)?;
        write_response(&mut writer, response)?;
    }

    Ok(())
}
