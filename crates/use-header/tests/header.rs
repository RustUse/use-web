use use_header::{
    Header, get_header, has_header, is_valid_header_name, normalize_header_name, parse_header_line,
    parse_headers, remove_header, set_header,
};

#[test]
fn normalizes_header_names() {
    assert_eq!(normalize_header_name(" Content-Type "), "content-type");
}

#[test]
fn validates_header_names() {
    assert!(is_valid_header_name("X-Trace-Id"));
    assert!(!is_valid_header_name("Bad Header"));
}

#[test]
fn parses_header_lines() {
    assert_eq!(
        parse_header_line("Content-Type: text/plain"),
        Some(Header {
            name: String::from("content-type"),
            value: String::from("text/plain"),
        })
    );
}

#[test]
fn parses_multiple_headers() {
    let headers = parse_headers("Content-Type: text/plain\nX-Trace: abc123\nBroken");

    assert_eq!(headers.len(), 2);
    assert!(has_header(&headers, "x-trace"));
}

#[test]
fn looks_up_headers() {
    let headers = parse_headers("Content-Type: text/plain\nX-Trace: abc123");

    assert_eq!(
        get_header(&headers, "CONTENT-TYPE").as_deref(),
        Some("text/plain")
    );
}

#[test]
fn sets_and_removes_headers() {
    let mut headers = parse_headers("Content-Type: text/plain");

    set_header(&mut headers, "X-Trace", "abc123");
    set_header(&mut headers, "Content-Type", "application/json");
    assert_eq!(
        get_header(&headers, "content-type").as_deref(),
        Some("application/json")
    );
    assert_eq!(get_header(&headers, "x-trace").as_deref(), Some("abc123"));

    remove_header(&mut headers, "x-trace");
    assert!(!has_header(&headers, "x-trace"));
}

#[test]
fn rejects_malformed_and_empty_input() {
    assert_eq!(parse_header_line("Broken Header"), None);
    assert_eq!(parse_header_line(""), None);
    assert!(parse_headers("").is_empty());
}
