# noematic

[![CI](https://github.com/henrytill/noematic/actions/workflows/ci.yml/badge.svg)](https://github.com/henrytill/noematic/actions/workflows/ci.yml)

## Development Notes

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
  "path": "/home/ht/src/noematic/_build/default/host/noematic.exe",
  "type": "stdio",
  "allowed_origins": ["chrome-extension://coeiohafhkjnkgibglpnbpojgcmacego/"]
}
```

> [!IMPORTANT]
> You will need to modify the `path` and `allowed_origins` values.
>
> The `name` and `type` values **must** match the example.
