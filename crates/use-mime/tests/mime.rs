use use_mime::{
    extension_from_mime, is_css_mime, is_html_mime, is_image_mime, is_json_mime, is_text_mime,
    is_xml_mime, looks_like_mime, mime_from_extension, parse_mime,
};

#[test]
fn parses_mime_types() {
    let mime = parse_mime("application/ld+json; charset=utf-8").unwrap();

    assert_eq!(mime.type_, "application");
    assert_eq!(mime.subtype, "ld");
    assert_eq!(mime.suffix.as_deref(), Some("json"));
}

#[test]
fn maps_extensions_to_mime() {
    assert_eq!(mime_from_extension("html"), Some("text/html"));
    assert_eq!(mime_from_extension(".svg"), Some("image/svg+xml"));
}

#[test]
fn maps_mime_to_extensions() {
    assert_eq!(extension_from_mime("application/json"), Some("json"));
    assert_eq!(extension_from_mime("image/jpeg"), Some("jpg"));
}

#[test]
fn detects_text_image_json_html_xml_and_css() {
    assert!(is_text_mime("text/plain"));
    assert!(is_image_mime("image/png"));
    assert!(is_json_mime("application/ld+json"));
    assert!(is_html_mime("text/html"));
    assert!(is_xml_mime("image/svg+xml"));
    assert!(is_css_mime("text/css"));
}

#[test]
fn handles_malformed_and_empty_input() {
    assert!(!looks_like_mime("broken"));
    assert_eq!(parse_mime("text/"), None);
    assert_eq!(parse_mime(""), None);
}
