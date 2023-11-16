use std::{
    io::{self, Read, Write},
    path::PathBuf,
};

use serde_json::Value;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    TryFromInt(std::num::TryFromIntError),
    MissingParent(PathBuf),
    MissingExe(PathBuf),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::TryFromInt(e) => write!(f, "TryFromInt error: {}", e),
            Error::MissingParent(path) => write!(f, "Missing parent: {}", path.display()),
            Error::MissingExe(path) => write!(f, "Missing exe: {}", path.display()),
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

impl From<std::num::TryFromIntError> for Error {
    fn from(other: std::num::TryFromIntError) -> Self {
        Error::TryFromInt(other)
    }
}

impl std::error::Error for Error {}

pub fn exe(name: &str) -> Result<PathBuf, Error> {
    let current_exe = std::env::current_exe()?;
    let current_dir = current_exe
        .parent()
        .ok_or_else(|| Error::MissingParent(current_exe.to_owned()))?;
    let mut path = current_dir
        .parent()
        .ok_or_else(|| Error::MissingParent(current_dir.to_owned()))
        .map(ToOwned::to_owned)?;
    path.push(name);
    path.set_extension(std::env::consts::EXE_EXTENSION);
    if path.exists() {
        Ok(path)
    } else {
        Err(Error::MissingExe(path))
    }
}

pub fn write_request(writer: &mut impl Write, request: &Value) -> Result<(), Error> {
    let request_bytes = request.to_string().into_bytes();
    let request_bytes_len = u32::try_from(request_bytes.len())?.to_ne_bytes();
    writer.write_all(&request_bytes_len)?;
    writer.write_all(&request_bytes)?;
    writer.flush()?;
    Ok(())
}

pub fn read_response(reader: &mut impl Read) -> Result<Value, Error> {
    let mut len_buf = [0; 4];
    reader.read_exact(&mut len_buf)?;
    let response_bytes_len = u32::from_ne_bytes(len_buf);
    let mut response_bytes = vec![0; response_bytes_len as usize];
    reader.read_exact(&mut response_bytes)?;
    serde_json::from_slice::<Value>(&response_bytes).map_err(Into::into)
}
