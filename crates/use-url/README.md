# use-url

Small URL utility primitives for `RustUse`.

## Experimental

`use-url` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_url::{extract_host, parse_url_basic, remove_query};

let parts = parse_url_basic("https://example.com/docs?page=2#intro").unwrap();

assert_eq!(parts.scheme, "https");
assert_eq!(extract_host("https://example.com/docs?page=2#intro").as_deref(), Some("example.com"));
assert_eq!(remove_query("https://example.com/docs?page=2#intro"), "https://example.com/docs#intro");
```

## Scope

- Basic URL splitting for scheme, host, port, path, query, and fragment.
- Lightweight path and suffix helpers.
- Graceful handling for malformed or partial input.

## Non-goals

- Full WHATWG URL parsing.
- DNS validation.
- HTTP client behavior.
- IDNA handling.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
