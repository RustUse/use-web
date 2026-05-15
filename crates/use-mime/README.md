# use-mime

Small MIME type utilities for `RustUse`.

## Experimental

`use-mime` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_mime::{is_json_mime, mime_from_extension, parse_mime};

let mime = parse_mime("application/ld+json; charset=utf-8").unwrap();

assert_eq!(mime.type_, "application");
assert_eq!(mime.subtype, "ld");
assert_eq!(mime.suffix.as_deref(), Some("json"));
assert_eq!(mime_from_extension("svg"), Some("image/svg+xml"));
assert!(is_json_mime("application/ld+json"));
```

## Scope

- Lightweight MIME parsing and classification.
- Small extension and MIME lookup tables for common web assets.
- Graceful handling for malformed input.

## Non-goals

- Complete IANA MIME coverage.
- Content sniffing.
- File parsing.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
