# use-uri

Small URI utility primitives for `RustUse`.

## Experimental

`use-uri` is experimental while the `use-web` workspace remains below `0.3.0`. Expect API adjustments as the first wave settles.

## Example

```rust
use use_uri::{extract_query, parse_uri_basic};

let parts = parse_uri_basic("https://example.com/docs?page=2#intro");

assert_eq!(parts.scheme.as_deref(), Some("https"));
assert_eq!(parts.authority.as_deref(), Some("example.com"));
assert_eq!(extract_query("https://example.com/docs?page=2#intro").as_deref(), Some("page=2"));
```

## Scope

- Small helpers for URI inspection and string-based splitting.
- Basic extraction for schemes, authorities, queries, and fragments.
- Predictable behavior for malformed or partial input.

## Non-goals

- Full RFC URI parsing.
- URL normalization.
- IDNA handling.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
