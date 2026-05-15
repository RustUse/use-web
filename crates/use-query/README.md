# use-query

Small query string utilities for `RustUse`.

## Experimental

`use-query` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_query::{append_query_param, get_query_param, parse_query};

let params = parse_query("page=2&draft");

assert_eq!(params.len(), 2);
assert_eq!(get_query_param("page=2&draft", "page").as_deref(), Some("2"));
assert_eq!(append_query_param("page=2", "lang", "en"), "page=2&lang=en");
```

## Scope

- Small parsing and building helpers for query strings.
- Lookup, removal, and append helpers for common tooling tasks.
- Graceful handling for repeated, empty, or missing values.

## Non-goals

- Full percent-encoding support.
- URL parser replacement.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
