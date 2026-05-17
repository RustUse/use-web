# use-web-css

Small CSS utility primitives for `RustUse`.

## Experimental

`use-web-css` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

The published package name is `use-web-css` so it avoids the existing crates.io
collision on `use-css`, while the Rust library target remains `use_css`.

## Example

```rust
use use_css::{extract_css_declarations, is_css_color_value, minify_css_basic};

let declarations = extract_css_declarations(".card { color: red; margin-top: 1rem; }");

assert_eq!(declarations.len(), 2);
assert!(is_css_color_value("#ff00aa"));
assert_eq!(minify_css_basic(".card { color: red; }"), ".card{color:red;}");
```

## Scope

- Lightweight CSS detection and declaration parsing.
- Small helpers for identifiers, custom properties, colors, and lengths.
- Comment stripping and basic minification.

## Non-goals

- Full CSS parsing.
- Cascade engines.
- Browser style engines.
- CSS-in-JS utilities.
- JavaScript tooling.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
