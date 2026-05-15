# use-header

Small HTTP header utilities for `RustUse`.

## Experimental

`use-header` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_header::{get_header, parse_headers};

let headers = parse_headers("Content-Type: text/plain\nX-Trace: abc123");

assert_eq!(get_header(&headers, "content-type").as_deref(), Some("text/plain"));
```

## Scope

- Header-name normalization and validation.
- Small helpers for parsing plain-text header blocks.
- Mutable helpers for setting and removing header values.

## Non-goals

- Full HTTP parsing.
- HTTP/2 or HTTP/3 frame handling.
- Client or server implementations.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
