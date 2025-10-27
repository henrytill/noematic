use std::{
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Error;
use serde_json::Value;

pub fn exe(name: &str) -> Result<PathBuf, Error> {
    let current_exe = std::env::current_exe()?;
    let current_dir = current_exe
        .parent()
        .ok_or_else(|| Error::msg(format!("Missing parent: {}", current_exe.to_string_lossy())))?;
    let mut path = current_dir
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| Error::msg(format!("Missing parent: {}", current_exe.to_string_lossy())))?;
    path.push(name);
    path.set_extension(std::env::consts::EXE_EXTENSION);
    if path.exists() {
        Ok(path)
    } else {
        Err(Error::msg(format!(
            "Missing executable: {}",
            path.to_string_lossy()
        )))
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
