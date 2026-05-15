# use-http

Small HTTP primitive utilities for `RustUse`.

## Experimental

`use-http` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_http::{HttpVersion, parse_request_line};

let request = parse_request_line("GET /docs HTTP/1.1").unwrap();

assert_eq!(request.method, "GET");
assert_eq!(request.target, "/docs");
assert_eq!(request.version, HttpVersion::Http11);
```

## Scope

- HTTP version parsing and formatting.
- Lightweight request-line and status-line parsing.
- Helpers for classifying request-target forms.

## Non-goals

- HTTP clients or servers.
- TCP or TLS handling.
- Async networking.
- Full HTTP parsers.
- Body parsing.
- HTTP/2 or HTTP/3 frame handling.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
