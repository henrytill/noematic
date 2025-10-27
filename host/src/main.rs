use std::{
    fs,
    io::{self, BufReader, BufWriter, Read, Write},
    mem,
};

use anyhow::Error;
use clap::Parser;
use directories::ProjectDirs;
use serde_json::Value;

use noematic::{
    Context,
    message::{MessageVersion, Request},
};

// We use unchecked casts to convert u32 to usize.
const _: () = assert!(mem::size_of::<usize>() >= mem::size_of::<u32>());

const MSG_MISSING_HOME_DIR: &str = "Missing home directory";
const MSG_UNSUPPORTED_VERSION: &str = "Unsupported version";
const MSG_UNSUPPORTED_LENGTH: &str = "Unsupported length";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run with in-memory database
    #[arg(short, long)]
    test: bool,
    /// Id
    id: Option<String>,
}

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

/// Reads a bytestring of the given length.
fn read_bytes(reader: &mut impl Read, length: u32) -> io::Result<Vec<u8>> {
    let length = length as usize;
    let mut ret = vec![0; length];
    reader.read_exact(&mut ret)?;
    Ok(ret)
}

/// Reads a length `n`-prefixed bytestring into a vector of length `n`.
///
/// Returns `None` if the reader is at EOF.
fn read_message_bytes(reader: &mut impl Read) -> Result<Option<Vec<u8>>, Error> {
    let length = match read_length(reader)? {
        Some(length) => length,
        None => return Ok(None),
    };
    read_bytes(reader, length).map(Some).map_err(Into::into)
}

/// Writes a bytestring, prefixed by its length, to the writer.
fn write_message_bytes(writer: &mut impl Write, bytes: &[u8]) -> Result<(), Error> {
    let len = u32::try_from(bytes.len())
        .or(Err(Error::msg(MSG_UNSUPPORTED_LENGTH)))?
        .to_ne_bytes();
    writer.write_all(&len)?;
    writer.write_all(bytes)?;
    writer.flush()?;
    Ok(())
}

fn get_project_dirs() -> Result<ProjectDirs, Error> {
    ProjectDirs::from("com.github", "henrytill", "noematic")
        .ok_or_else(|| Error::msg(MSG_MISSING_HOME_DIR))
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let mut context = if args.test {
        Context::in_memory()?
    } else {
        let db_path = {
            let project_dirs: ProjectDirs = get_project_dirs()?;
            let data_dir = project_dirs.data_dir();
            fs::create_dir_all(data_dir)?;
            data_dir.join("db.sqlite3")
        };
        Context::persistent(db_path)?
    };

    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());

    while let Some(message_bytes) = read_message_bytes(&mut reader)? {
        let message_json: Value = serde_json::from_slice(&message_bytes)?;

        let version = noematic::extract_version(&message_json)?;
        if version != MessageVersion::EXPECTED {
            return Err(Error::msg(MSG_UNSUPPORTED_VERSION));
        }

        let request: Request = serde_json::from_value(message_json)?;
        let responses = noematic::handle_request(&mut context, request)?;
        for response in responses {
            let response_bytes = serde_json::to_string(&response)?.into_bytes();
            write_message_bytes(&mut writer, &response_bytes)?;
        }
    }

    Ok(())
}
