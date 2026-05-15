# use-cookie

Small cookie utilities for `RustUse`.

## Experimental

`use-cookie` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_cookie::{get_cookie, parse_set_cookie_basic};

assert_eq!(get_cookie("session=abc; theme=dark", "theme").as_deref(), Some("dark"));
assert!(parse_set_cookie_basic("session=abc; Path=/; HttpOnly").is_some());
```

## Scope

- Cookie header parsing and formatting.
- Lightweight `Set-Cookie` inspection for common attributes.
- Small validation helpers for cookie names.

## Non-goals

- Browser cookie jars.
- Public suffix handling.
- Security policy engines.
- Session management.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
