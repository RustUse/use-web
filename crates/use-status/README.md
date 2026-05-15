# use-status

Small HTTP status utilities for `RustUse`.

## Experimental

`use-status` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_status::{is_error, reason_phrase, status_class};

assert_eq!(reason_phrase(404), Some("Not Found"));
assert_eq!(status_class(201), Some(use_status::StatusClass::Success));
assert!(is_error(503));
```

## Scope

- Reason-phrase lookup for common status codes.
- Classification helpers for response classes.
- Small utility predicates for application and tooling code.

## Non-goals

- HTTP server implementations.
- HTTP client implementations.
- Policy engines.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
