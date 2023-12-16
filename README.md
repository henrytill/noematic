<div align="center">
  <h1>Noematic</h1>
  <p><strong>Search your backlog</strong></p>
  <p>
    <a href="https://github.com/henrytill/noematic/actions/workflows/ci.yml"><img src="https://github.com/henrytill/noematic/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  </p>
</div>

## About

Noematic is an experimental browser extension that reimagines bookmarking. When you bookmark a page, its text content is saved locally to your computer. This content is indexed, enabling bookmarks to be retrieved through full-text search.

## Status

> [!WARNING]
> This is a work-in-progress prototype. It is unstable and not yet fit for general use.

## Development Notes

### Architecture

Noematic is currently comprised of two components:

- a web extension, usable in Firefox or Chromium browsers
- a native executable which communicates with the extension using [native messaging](https://developer.chrome.com/docs/extensions/mv3/nativeMessaging/).

### Build

On Linux:

```sh
make
```

The extension build products can be loaded from:

```
extension/dist/firefox
extension/dist/chromium
```

### Native Manifest File

Refer to the native messaging documentation for [Firefox](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests) or [Chromium](https://developer.chrome.com/docs/extensions/mv3/nativeMessaging/) for more information, including manifest file locations for various platforms.

On Linux, native manifests for both browsers can installed using the following command:

```sh
make create_host_manifest
```

The native manifests point directly to the debug executable.

### Useful Documentation

- <https://developer.chrome.com/docs/extensions/reference/runtime/>
- <https://developer.chrome.com/docs/extensions/mv3/messaging/>
- <https://developer.chrome.com/docs/extensions/mv3/nativeMessaging/>
