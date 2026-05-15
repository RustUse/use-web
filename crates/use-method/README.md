# use-method

Small HTTP method utilities for `RustUse`.

## Experimental

`use-method` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_method::{is_safe_method, parse_method};

assert!(is_safe_method("GET"));
assert_eq!(parse_method("patch"), Some(use_method::HttpMethod::Patch));
```

## Scope

- Lightweight method parsing and normalization.
- Small helpers for HTTP semantics such as safety and idempotency.
- Graceful handling for unknown but valid method tokens.

## Non-goals

- Routing.
- Middleware.
- Request execution.
- Policy engines.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
