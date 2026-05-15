# use-origin

Small web origin utilities for `RustUse`.

## Experimental

`use-origin` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_origin::{extract_origin, is_secure_origin, same_origin};

assert!(same_origin("https://example.com/docs", "https://example.com/api"));
assert!(is_secure_origin("https://example.com"));
assert_eq!(extract_origin("https://example.com/docs").unwrap().host, "example.com");
```

## Scope

- Basic origin parsing and extraction.
- Same-origin comparison with default-port awareness.
- Small helpers for formatting and secure-origin checks.

## Non-goals

- Browser security models.
- CORS engines.
- TLS validation.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
