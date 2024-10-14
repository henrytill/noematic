mod base;

use std::process::{Command, Stdio};

use serde_json::json;

const VERSION: &str = "0.1.0";
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
        "version": VERSION,
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
        "version": VERSION,
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
        "version": VERSION,
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
        "version": VERSION,
        "action": "saveResponse",
        "payload": {},
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": VERSION,
        "action": "searchRequest",
        "payload": {
            "query": "quux",
            "pageNum": 0,
            "pageLength": 10,
        },
        "correlationId": CORRELATION_ID
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &search_request).expect("Failed to write request");

    let expected = json!({
        "version": VERSION,
        "action": "searchResponseHeader",
        "payload": {
            "query": "quux",
            "pageNum": 0,
            "pageLength": 1,
            "hasMore": false,
        },
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let expected = json!({
        "version": VERSION,
        "action": "searchResponseSite",
        "payload": {
            "url": "https://en.wikipedia.org/wiki/Foobar",
            "title": "Title",
            "snippet": "Foo bar baz <b>quux</b>",
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
        "version": VERSION,
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
        "version": VERSION,
        "action": "saveResponse",
        "payload": {},
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": VERSION,
        "action": "searchRequest",
        "payload": {
            "query": "\"\"foo-\"***bar\"\"",
            "pageNum": 0,
            "pageLength": 10,
        },
        "correlationId": CORRELATION_ID
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &search_request).expect("Failed to write request");

    let expected = json!({
        "version": VERSION,
        "action": "searchResponseHeader",
        "payload": {
            "query": "\"\"foo-\"***bar\"\"",
            "pageNum": 0,
            "pageLength": 1,
            "hasMore": false,
        },
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let expected = json!({
        "version": VERSION,
        "action": "searchResponseSite",
        "payload": {
            "url": "https://en.wikipedia.org/wiki/Foobar",
            "title": "Title",
            "snippet": "<b>foo</b> <b>bar</b> baz quux",
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
        "version": VERSION,
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
        "version": VERSION,
        "action": "saveResponse",
        "payload": {},
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": VERSION,
        "action": "searchRequest",
        "payload": {
            "query": "quux",
            "pageNum": 0,
            "pageLength": 10,
        },
        "correlationId": CORRELATION_ID
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &search_request).expect("Failed to write request");

    let expected_header = json!({
        "version": VERSION,
        "action": "searchResponseHeader",
        "payload": {
            "query": "quux",
            "pageNum": 0,
            "pageLength": 1,
            "hasMore": false,
        },
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected_header, actual);

    let expected_site = json!({
        "version": VERSION,
        "action": "searchResponseSite",
        "payload": {
            "url": "https://en.wikipedia.org/wiki/Foobar",
            "title": "Title",
            "snippet": "Foo bar baz <b>quux</b>",
        },
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected_site, actual);

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &search_request).expect("Failed to write request");

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected_header, actual);

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected_site, actual);
}

#[test]
fn test_remove() {
    let noematic = base::exe("noematic").unwrap();
    let mut child = Command::new(noematic)
        .arg("-test")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let request = json!({
        "version": VERSION,
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

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let _ = base::read_response(stdout).expect("Failed to read response");

    let request = json!({
        "version": VERSION,
        "action": "removeRequest",
        "payload": {
            "url": "https://en.wikipedia.org/wiki/Foobar",
        },
        "correlationId": CORRELATION_ID
    });
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &request).expect("Failed to write request");

    let expected = json!({
        "version": VERSION,
        "action": "removeResponse",
        "payload": {},
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);

    let search_request = json!({
        "version": VERSION,
        "action": "searchRequest",
        "payload": {
            "query": "quux",
            "pageNum": 0,
            "pageLength": 10,
        },
        "correlationId": CORRELATION_ID
    });

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    base::write_request(stdin, &search_request).expect("Failed to write request");

    let expected = json!({
        "version": VERSION,
        "action": "searchResponseHeader",
        "payload": {
            "query": "quux",
            "pageNum": 0,
            "pageLength": 0,
            "hasMore": false,
        },
        "correlationId": CORRELATION_ID
    });

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let actual = base::read_response(stdout).expect("Failed to read response");

    assert_eq!(expected, actual);
}
