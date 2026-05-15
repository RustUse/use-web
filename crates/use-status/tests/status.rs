use use_status::{
    StatusClass, is_cacheable_by_default, is_client_error, is_error, is_redirect, is_server_error,
    is_success, reason_phrase, status_class,
};

#[test]
fn looks_up_reason_phrases() {
    assert_eq!(reason_phrase(200), Some("OK"));
    assert_eq!(reason_phrase(404), Some("Not Found"));
}

#[test]
fn detects_status_classes() {
    assert_eq!(status_class(101), Some(StatusClass::Informational));
    assert_eq!(status_class(201), Some(StatusClass::Success));
    assert_eq!(status_class(302), Some(StatusClass::Redirection));
    assert_eq!(status_class(422), Some(StatusClass::ClientError));
    assert_eq!(status_class(503), Some(StatusClass::ServerError));
}

#[test]
fn detects_success_and_redirects() {
    assert!(is_success(204));
    assert!(is_redirect(308));
}

#[test]
fn detects_errors() {
    assert!(is_client_error(404));
    assert!(is_server_error(503));
    assert!(is_error(500));
    assert!(!is_error(200));
}

#[test]
fn detects_default_cacheability() {
    assert!(is_cacheable_by_default(200));
    assert!(is_cacheable_by_default(404));
    assert!(!is_cacheable_by_default(302));
}

#[test]
fn handles_unknown_status_codes() {
    assert_eq!(reason_phrase(999), None);
    assert_eq!(status_class(99), None);
}
