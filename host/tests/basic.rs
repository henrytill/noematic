use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde_json::{json, Value};

#[derive(Debug)]
enum Error {
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

fn write_request(writer: &mut impl Write, request: &Value) -> Result<(), Error> {
    let request_bytes = request.to_string().into_bytes();
    let request_bytes_len = u32::try_from(request_bytes.len())?.to_ne_bytes();
    writer.write_all(&request_bytes_len)?;
    writer.write_all(&request_bytes)?;
    writer.flush()?;
    Ok(())
}

fn read_response(reader: &mut impl Read) -> Result<Value, Error> {
    let mut len_buf = [0; 4];
    reader.read_exact(&mut len_buf)?;
    let response_bytes_len = u32::from_ne_bytes(len_buf);
    let mut response_bytes = vec![0; response_bytes_len as usize];
    reader.read_exact(&mut response_bytes)?;
    serde_json::from_slice::<Value>(&response_bytes).map_err(Into::into)
}

#[test]
fn test_save() {
    let noematic = exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let request = json!({
        "version": "0.1.0",
        "action": "saveRequest",
        "payload": {
            "url": "https://en.wikipedia.org/wiki/Foobar",
            "title": "Title",
            "innerText": "Inner text"
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    write_request(stdin, &request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "saveResponse",
        "payload": {},
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });
    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);
}

#[test]
fn test_search() {
    let noematic = exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let save_request = json!({
        "version": "0.1.0",
        "action": "saveRequest",
        "payload": {
            "url": "https://en.wikipedia.org/wiki/Foobar",
            "title": "Title",
            "innerText": "Foo bar baz quux"
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    write_request(stdin, &save_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "saveResponse",
        "payload": {},
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": "0.1.0",
        "action": "searchRequest",
        "payload": {
            "query": "quux"
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    write_request(stdin, &search_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "searchResponse",
        "payload": {
            "results": [
                {
                    "url": "https://en.wikipedia.org/wiki/Foobar",
                    "title": "Title",
                    "innerText": "Foo bar baz quux",
                }
            ]
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);
}

#[test]
fn test_search_quotation() {
    let noematic = exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let save_request = json!({
        "version": "0.1.0",
        "action": "saveRequest",
        "payload": {
            "url": "https://en.wikipedia.org/wiki/Foobar",
            "title": "Title",
            "innerText": "foo bar baz quux"
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    write_request(stdin, &save_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "saveResponse",
        "payload": {},
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": "0.1.0",
        "action": "searchRequest",
        "payload": {
            "query": "\"\"foo-\"***bar\"\""
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    write_request(stdin, &search_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "searchResponse",
        "payload": {
            "results": [
                {
                    "url": "https://en.wikipedia.org/wiki/Foobar",
                    "title": "Title",
                    "innerText": "foo bar baz quux",
                }
            ]
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);
}

#[test]
fn search_idempotent() {
    let noematic = exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let save_request = json!({
        "version": "0.1.0",
        "action": "saveRequest",
        "payload": {
            "url": "https://en.wikipedia.org/wiki/Foobar",
            "title": "Title",
            "innerText": "Foo bar baz quux"
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    write_request(stdin, &save_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "saveResponse",
        "payload": {},
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": "0.1.0",
        "action": "searchRequest",
        "payload": {
            "query": "quux"
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    write_request(stdin, &search_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "searchResponse",
        "payload": {
            "results": [
                {
                    "url": "https://en.wikipedia.org/wiki/Foobar",
                    "title": "Title",
                    "innerText": "Foo bar baz quux",
                }
            ]
        },
        "correlationId": "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    write_request(stdin, &search_request).expect("Failed to write request");

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);
}
