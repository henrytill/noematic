<div align="center">
  <h1>noematic</h1>
  <p><strong>Search your backlog</strong></p>
  <p>
    <a href="https://github.com/henrytill/noematic/actions/workflows/ci.yml"><img src="https://github.com/henrytill/noematic/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  </p>
</div>

## About

`noematic` is an experimental browser extension that reimagines bookmarking. When you bookmark a page, its text content is saved locally to your computer. This content is indexed, enabling bookmarks to be retrieved through full-text search.

## Status

> [!WARNING]
> This is a work-in-progress prototype. It is unstable and not yet fit for general use.

Currently, it only works with Chrome/Chromium. Firefox support is planned.

## Development Notes

### Architecture

`noematic` is currently comprised of two components: a web extension, and a native executable which communicates with the extension using [native messaging](https://developer.chrome.com/docs/extensions/mv3/nativeMessaging/).

### Native Messaging Host Manifest File

Refer to the [native messaging documentation](https://developer.chrome.com/docs/extensions/mv3/nativeMessaging/) for more information, including manifest file locations for various platforms.

The manifest file should be named:

```
com.github.henrytill.noematic.json
```

It should look like:

```json
{
  "name": "com.github.henrytill.noematic",
  "description": "Search your backlog",
  "path": "/home/ht/src/noematic/host/target/debug/noematic",
  "type": "stdio",
  "allowed_origins": ["chrome-extension://nhpmniahkjglmdbkbbolkakgfbgdiplj/"]
}
```

> [!IMPORTANT]
> You will need to modify the `path` and `allowed_origins` values.
>
> The `name` and `type` values **must** match the example.

### Documentation

- <https://developer.chrome.com/docs/extensions/reference/runtime/>
- <https://developer.chrome.com/docs/extensions/mv3/messaging/>
- <https://developer.chrome.com/docs/extensions/mv3/nativeMessaging/>
