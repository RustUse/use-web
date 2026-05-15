use use_http::{
    HttpVersion, format_http_version, is_http_version, is_request_target_absolute_form,
    is_request_target_asterisk_form, is_request_target_authority_form,
    is_request_target_origin_form, looks_like_http_request_line, looks_like_http_status_line,
    parse_http_version, parse_request_line, parse_status_line,
};

#[test]
fn parses_http_versions() {
    assert_eq!(parse_http_version("HTTP/1.1"), HttpVersion::Http11);
    assert_eq!(parse_http_version("http/3"), HttpVersion::Http3);
    assert!(is_http_version("HTTP/2"));
}

#[test]
fn formats_http_versions() {
    assert_eq!(format_http_version(HttpVersion::Http10), "HTTP/1.0");
    assert_eq!(format_http_version(HttpVersion::Unknown), "HTTP/?");
}

#[test]
fn detects_and_parses_request_lines() {
    assert!(looks_like_http_request_line("GET /docs HTTP/1.1"));

    let request = parse_request_line("GET /docs HTTP/1.1").unwrap();
    assert_eq!(request.method, "GET");
    assert_eq!(request.target, "/docs");
    assert_eq!(request.version, HttpVersion::Http11);
}

#[test]
fn detects_and_parses_status_lines() {
    assert!(looks_like_http_status_line("HTTP/1.1 404 Not Found"));

    let status = parse_status_line("HTTP/1.1 404 Not Found").unwrap();
    assert_eq!(status.version, HttpVersion::Http11);
    assert_eq!(status.code, 404);
    assert_eq!(status.reason.as_deref(), Some("Not Found"));
}

#[test]
fn detects_request_target_forms() {
    assert!(is_request_target_origin_form("/items?limit=10"));
    assert!(is_request_target_absolute_form("https://example.com/items"));
    assert!(is_request_target_authority_form("example.com:443"));
    assert!(is_request_target_asterisk_form("*"));
}

#[test]
fn rejects_malformed_input() {
    assert!(!looks_like_http_request_line("GET /docs"));
    assert_eq!(parse_request_line("GET /docs"), None);
    assert_eq!(parse_status_line("HTTP/1.1 two hundred OK"), None);
    assert!(!is_request_target_authority_form("example.com"));
}

#[test]
fn rejects_empty_input() {
    assert_eq!(parse_request_line(""), None);
    assert_eq!(parse_status_line(""), None);
    assert!(!looks_like_http_status_line(""));
}
