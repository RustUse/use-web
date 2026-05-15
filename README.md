# RustUse use-web

`use-web` is a RustUse workspace for small, focused Rust 2024 utility crates around web, HTTP, browser-adjacent, and API primitives.

## Experimental

Every crate in this workspace is experimental while the release line remains below `0.3.0`. Expect incremental API cleanup as the first wave settles.

## Crate List

- `use-web`: feature-gated umbrella crate for the full workspace
- `use-http`: HTTP request-line, status-line, and version helpers
- `use-method`: HTTP method parsing and classification helpers
- `use-header`: HTTP header parsing and mutation helpers
- `use-status`: HTTP status lookup and classification helpers
- `use-cookie`: cookie and `Set-Cookie` helpers
- `use-mime`: MIME parsing, lookup, and classification helpers
- `use-uri`: URI inspection and splitting helpers
- `use-url`: URL inspection, host, port, path, query, and fragment helpers
- `use-query`: query-string parsing and building helpers
- `use-origin`: origin extraction and same-origin helpers
- `use-route`: route normalization and simple `:param` matching helpers
- `use-html`: lightweight HTML extraction helpers
- `use-css`: lightweight CSS declaration and value helpers

## Scope

- Lightweight primitives and string-based helpers for common web-adjacent tasks.
- Small composable APIs for CLIs, docs tooling, static sites, fixtures, and application glue code.
- Graceful handling for malformed input without panics.

## Non-goals

- Full web frameworks.
- HTTP clients or servers.
- Full HTTP, HTML, or CSS parsers.
- Browser engines or runtime behavior.
- JavaScript language utilities.
- Low-level network I/O.

## Boundary

`use-web` covers application and API primitives such as HTTP, methods, headers, statuses, cookies, MIME types, URLs, URIs, query strings, origins, routes, HTML, and CSS.

`use-net` should cover lower-level network primitives such as IP addresses, hosts, domains, DNS, sockets, TCP, UDP, CIDR, MAC addresses, and related transport concerns.

## Example Usage

```toml
[dependencies]
use-web = { version = "0.0.1", default-features = false, features = ["url", "status", "html"] }
```

```rust
use use_html::extract_title;
use use_status::reason_phrase;
use use_url::parse_url_basic;

let parts = parse_url_basic("https://example.com/docs?page=2#intro").unwrap();

assert_eq!(parts.host.as_deref(), Some("example.com"));
assert_eq!(reason_phrase(404), Some("Not Found"));
assert_eq!(extract_title("<title>Docs</title>").as_deref(), Some("Docs"));
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
