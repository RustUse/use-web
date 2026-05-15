# use-route

Small route and path utilities for `RustUse`.

## Experimental

`use-route` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_route::{extract_route_params, matches_route_pattern, normalize_route};

assert_eq!(normalize_route("//users//42/"), "/users/42");
assert!(matches_route_pattern("/users/:id", "/users/42"));
assert_eq!(extract_route_params("/users/:id", "/users/42")[0].value, "42");
```

## Scope

- Route normalization and joining.
- Simple segment splitting and depth counting.
- Lightweight `:param` pattern matching.

## Non-goals

- Full routers.
- Middleware.
- Regex route engines.
- Server implementations.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
