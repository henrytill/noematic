use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde_json::{json, Value};

#[derive(Debug)]
enum Error {
    Io(io::Error),
    MissingParent(PathBuf),
    MissingExe(PathBuf),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
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

impl std::error::Error for Error {}

fn exe(name: &str) -> Result<PathBuf, Error> {
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

#[test]
fn test_integration() {
    let noematic = exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let request = json!({
        "version": 1,
        "action": "saveRequest",
        "payload": { "title": "Title", "innerText": "Inner text" },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });
    let request_bytes = request.to_string().into_bytes();
    let response_length = TryInto::<u32>::try_into(request_bytes.len())
        .unwrap()
        .to_ne_bytes();

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(&response_length)
            .expect("Failed to write length");
        stdin
            .write_all(&request_bytes)
            .expect("Failed to write body");
        stdin.flush().expect("Failed to flush");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    let expected = json!({
        "version": 1,
        "action": "saveResponse",
        "payload": { "status": "Success", "details": "Item saved" },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });
    let expected_length = TryInto::<u32>::try_into(expected.to_string().into_bytes().len())
        .unwrap()
        .to_ne_bytes();

    let actual = serde_json::from_slice::<Value>(&output.stdout[4..]).unwrap();
    let actual_length = &output.stdout[0..4];

    assert!(output.status.success());
    assert_eq!(expected_length, actual_length);
    assert_eq!(expected, actual);
}
