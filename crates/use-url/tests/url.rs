use use_url::{
    ensure_trailing_slash, extract_host, extract_path, extract_port, is_http_url, is_https_url,
    looks_like_url, parse_url_basic, remove_fragment, remove_query, strip_trailing_slash,
};

#[test]
fn detects_http_and_https_urls() {
    assert!(is_http_url("http://example.com"));
    assert!(is_https_url("https://example.com"));
    assert!(looks_like_url("https://example.com/docs"));
}

#[test]
fn parses_basic_url_parts() {
    let parts = parse_url_basic("https://example.com:8443/docs?page=2#intro").unwrap();

    assert_eq!(parts.scheme, "https");
    assert_eq!(parts.host.as_deref(), Some("example.com"));
    assert_eq!(parts.port, Some(8443));
    assert_eq!(parts.path, "/docs");
    assert_eq!(parts.query.as_deref(), Some("page=2"));
    assert_eq!(parts.fragment.as_deref(), Some("intro"));
}

#[test]
fn extracts_host_port_and_path() {
    let url = "https://example.com:8443/docs?page=2#intro";

    assert_eq!(extract_host(url).as_deref(), Some("example.com"));
    assert_eq!(extract_port(url), Some(8443));
    assert_eq!(extract_path(url).as_deref(), Some("/docs"));
}

#[test]
fn supports_trailing_slash_helpers() {
    assert_eq!(
        ensure_trailing_slash("https://example.com/docs"),
        "https://example.com/docs/"
    );
    assert_eq!(
        ensure_trailing_slash("https://example.com?x=1"),
        "https://example.com/?x=1"
    );
    assert_eq!(
        strip_trailing_slash("https://example.com/docs/?x=1"),
        "https://example.com/docs?x=1"
    );
}

#[test]
fn removes_query_and_fragment() {
    assert_eq!(
        remove_query("https://example.com/docs?page=2#intro"),
        "https://example.com/docs#intro"
    );
    assert_eq!(
        remove_fragment("https://example.com/docs?page=2#intro"),
        "https://example.com/docs?page=2"
    );
}

#[test]
fn handles_malformed_and_empty_input() {
    assert_eq!(parse_url_basic("mailto:user@example.com"), None);
    assert_eq!(parse_url_basic(""), None);
    assert!(!looks_like_url("not a url"));
}
