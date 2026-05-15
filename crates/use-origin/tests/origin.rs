use use_origin::{
    default_port_for_scheme, extract_origin, format_origin, is_secure_origin, parse_origin,
    same_origin,
};

#[test]
fn parses_origins() {
    let origin = parse_origin("https://example.com:8443").unwrap();

    assert_eq!(origin.scheme, "https");
    assert_eq!(origin.host, "example.com");
    assert_eq!(origin.port, Some(8443));
    assert_eq!(format_origin(&origin), "https://example.com:8443");
}

#[test]
fn extracts_origins_from_urls() {
    let origin = extract_origin("https://example.com/docs?page=2").unwrap();
    assert_eq!(origin.host, "example.com");
}

#[test]
fn compares_same_origins() {
    assert!(same_origin(
        "https://example.com/docs",
        "https://example.com/api"
    ));
    assert!(same_origin(
        "https://example.com",
        "https://example.com:443/docs"
    ));
    assert!(!same_origin("https://example.com", "http://example.com"));
}

#[test]
fn detects_secure_origins_and_default_ports() {
    assert!(is_secure_origin("https://example.com"));
    assert!(!is_secure_origin("http://example.com"));
    assert_eq!(default_port_for_scheme("https"), Some(443));
}

#[test]
fn handles_malformed_and_empty_input() {
    assert_eq!(parse_origin("https://example.com/docs"), None);
    assert_eq!(extract_origin("not an origin"), None);
    assert_eq!(extract_origin(""), None);
}
