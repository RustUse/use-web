use use_method::{
    HttpMethod, is_idempotent_method, is_safe_method, is_standard_method, method_allows_body,
    method_expects_body, normalize_method, parse_method,
};

#[test]
fn parses_methods() {
    assert_eq!(parse_method("GET"), Some(HttpMethod::Get));
    assert_eq!(parse_method("patch"), Some(HttpMethod::Patch));
}

#[test]
fn normalizes_methods() {
    assert_eq!(normalize_method("  post  ").as_deref(), Some("POST"));
}

#[test]
fn detects_standard_methods() {
    assert!(is_standard_method("DELETE"));
    assert!(!is_standard_method("BREW"));
}

#[test]
fn detects_safe_and_idempotent_methods() {
    assert!(is_safe_method("HEAD"));
    assert!(!is_safe_method("POST"));
    assert!(is_idempotent_method("PUT"));
    assert!(!is_idempotent_method("PATCH"));
}

#[test]
fn detects_body_semantics() {
    assert!(method_allows_body("POST"));
    assert!(!method_allows_body("GET"));
    assert!(method_expects_body("PATCH"));
    assert!(!method_expects_body("OPTIONS"));
}

#[test]
fn handles_unknown_methods() {
    assert_eq!(
        parse_method("PURGE"),
        Some(HttpMethod::Other(String::from("PURGE")))
    );
    assert!(method_allows_body("PURGE"));
}

#[test]
fn rejects_empty_input() {
    assert_eq!(parse_method(""), None);
    assert_eq!(normalize_method("   "), None);
}
