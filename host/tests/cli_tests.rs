mod base;

use std::process::{Command, Stdio};

use serde_json::json;

const CORRELATION_ID: &str = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";

#[test]
fn test_save() {
    let noematic = base::exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .arg("-test")
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
        "correlationId": CORRELATION_ID
    });
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "saveResponse",
        "payload": {},
        "correlationId": CORRELATION_ID
    });
    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);
}

#[test]
fn test_search() {
    let noematic = base::exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .arg("-test")
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
        "correlationId": CORRELATION_ID
    });
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &save_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "saveResponse",
        "payload": {},
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": "0.1.0",
        "action": "searchRequest",
        "payload": {
            "query": "quux"
        },
        "correlationId": CORRELATION_ID
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &search_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "searchResponse",
        "payload": {
            "query": "quux",
            "results": [
                {
                    "url": "https://en.wikipedia.org/wiki/Foobar",
                    "title": "Title",
                    "innerText": "Foo bar baz quux",
                }
            ]
        },
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);
}

#[test]
fn test_search_quotation() {
    let noematic = base::exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .arg("-test")
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
        "correlationId": CORRELATION_ID
    });
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &save_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "saveResponse",
        "payload": {},
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": "0.1.0",
        "action": "searchRequest",
        "payload": {
            "query": "\"\"foo-\"***bar\"\""
        },
        "correlationId": CORRELATION_ID
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &search_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "searchResponse",
        "payload": {
            "query": "\"\"foo-\"***bar\"\"",
            "results": [
                {
                    "url": "https://en.wikipedia.org/wiki/Foobar",
                    "title": "Title",
                    "innerText": "foo bar baz quux",
                }
            ]
        },
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);
}

#[test]
fn search_idempotent() {
    let noematic = base::exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .arg("-test")
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
        "correlationId": CORRELATION_ID
    });
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &save_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "saveResponse",
        "payload": {},
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": "0.1.0",
        "action": "searchRequest",
        "payload": {
            "query": "quux"
        },
        "correlationId": CORRELATION_ID
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &search_request).expect("Failed to write request");

    let expected = json!({
        "version": "0.1.0",
        "action": "searchResponse",
        "payload": {
            "query": "quux",
            "results": [
                {
                    "url": "https://en.wikipedia.org/wiki/Foobar",
                    "title": "Title",
                    "innerText": "Foo bar baz quux",
                }
            ]
        },
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &search_request).expect("Failed to write request");

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);
}
