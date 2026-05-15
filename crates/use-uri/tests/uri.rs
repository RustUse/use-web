use use_uri::{
    extract_fragment, extract_query, extract_scheme, has_scheme, looks_like_uri, parse_uri_basic,
    strip_fragment, strip_query,
};

#[test]
fn detects_schemes() {
    assert!(has_scheme("https://example.com"));
    assert_eq!(
        extract_scheme("MailTo:user@example.com").as_deref(),
        Some("mailto")
    );
    assert!(looks_like_uri("custom+scheme:data"));
}

#[test]
fn extracts_query_and_fragment() {
    assert_eq!(
        extract_query("https://example.com/path?x=1&y=2#part").as_deref(),
        Some("x=1&y=2")
    );
    assert_eq!(
        extract_fragment("https://example.com/path?x=1&y=2#part").as_deref(),
        Some("part")
    );
    assert_eq!(
        strip_query("https://example.com/path?x=1&y=2#part"),
        "https://example.com/path"
    );
    assert_eq!(
        strip_fragment("https://example.com/path?x=1&y=2#part"),
        "https://example.com/path?x=1&y=2"
    );
}

#[test]
fn parses_basic_uri_parts() {
    let parts = parse_uri_basic("https://example.com/docs/index.html?lang=en#intro");

    assert_eq!(parts.scheme.as_deref(), Some("https"));
    assert_eq!(parts.authority.as_deref(), Some("example.com"));
    assert_eq!(parts.path, "/docs/index.html");
    assert_eq!(parts.query.as_deref(), Some("lang=en"));
    assert_eq!(parts.fragment.as_deref(), Some("intro"));
}

#[test]
fn handles_malformed_input_gracefully() {
    let parts = parse_uri_basic("://broken");

    assert_eq!(parts.scheme, None);
    assert_eq!(parts.authority, None);
    assert_eq!(parts.path, "://broken");
    assert_eq!(parts.query, None);
    assert_eq!(parts.fragment, None);
    assert!(!looks_like_uri("://broken"));
}

#[test]
fn handles_empty_input() {
    let parts = parse_uri_basic("");

    assert_eq!(parts.scheme, None);
    assert_eq!(parts.authority, None);
    assert_eq!(parts.path, "");
    assert_eq!(parts.query, None);
    assert_eq!(parts.fragment, None);
    assert!(!looks_like_uri(""));
}
