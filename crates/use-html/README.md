# use-html

Small HTML utility primitives for `RustUse`.

## Experimental

`use-html` is experimental while the `use-web` workspace remains below `0.3.0`. Expect small API adjustments during the first release wave.

## Example

```rust
use use_html::{extract_links, extract_title, strip_tags_basic};

let html = "<html><head><title>Docs</title></head><body><a href=\"/guide\">Guide</a></body></html>";

assert_eq!(extract_title(html).as_deref(), Some("Docs"));
assert_eq!(extract_links(html)[0].href, "/guide");
assert_eq!(strip_tags_basic("<p>Hello <strong>world</strong></p>"), "Hello world");
```

## Scope

- Lightweight HTML detection and string-based extraction helpers.
- Small helpers for links, headings, titles, meta tags, and attributes.
- Basic escaping and unescaping for common entities.

## Non-goals

- Full HTML parsing.
- DOM implementations.
- Browser compatibility layers.
- Sanitizers for security boundaries.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
