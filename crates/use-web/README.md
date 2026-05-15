# use-web

Feature-gated facade crate for the focused `RustUse` web primitives.

## Experimental

`use-web` is experimental while the workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
# fn main() {
# #[cfg(all(feature = "url", feature = "status", feature = "html"))]
# {
use use_web::{parse_url_basic, reason_phrase};

let parts = parse_url_basic("https://example.com/docs?page=2#intro").unwrap();

assert_eq!(parts.host.as_deref(), Some("example.com"));
assert_eq!(reason_phrase(404), Some("Not Found"));
assert_eq!(use_web::html::extract_title("<title>Docs</title>").as_deref(), Some("Docs"));
# }
# }
```

## Scope

- Thin re-exports over the focused crates in this workspace.
- Feature flags that map directly to the child crates.
- One dependency when consumer ergonomics matter more than the narrowest possible dependency graph.

## Non-goals

- A second abstraction layer over the focused crates.
- Framework behavior, networking, or browser runtime logic.
- JavaScript language utilities.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
