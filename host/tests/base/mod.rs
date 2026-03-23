use std::{
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Error;
use serde_json::Value;

/// Returns the path to the `noematic` binary under test.
///
/// Uses `CARGO_BIN_EXE_noematic`, which Cargo sets at compile time for
/// integration tests. This correctly handles custom build directories
/// (e.g. `build.build-dir` config or new Cargo layout features) unlike
/// manually deriving the path from `current_exe`.
pub fn exe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_noematic"))
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
