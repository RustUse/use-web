use use_cookie::{
    Cookie, build_cookie_header, cookie_name_is_valid, get_cookie, has_cookie, parse_cookie_header,
    parse_set_cookie_basic,
};

#[test]
fn parses_cookie_headers() {
    let cookies = parse_cookie_header("session=abc; theme=dark; empty=");

    assert_eq!(cookies.len(), 3);
    assert_eq!(cookies[0].name, "session");
    assert_eq!(cookies[1].value, "dark");
    assert_eq!(cookies[2].value, "");
}

#[test]
fn parses_set_cookie_headers() {
    let cookie = parse_set_cookie_basic(
        "session=abc; Path=/; Domain=example.com; Max-Age=60; Secure; HttpOnly; SameSite=Lax",
    )
    .unwrap();

    assert_eq!(cookie.name, "session");
    assert_eq!(cookie.value, "abc");
    assert_eq!(cookie.path.as_deref(), Some("/"));
    assert_eq!(cookie.domain.as_deref(), Some("example.com"));
    assert_eq!(cookie.max_age, Some(60));
    assert!(cookie.secure);
    assert!(cookie.http_only);
    assert_eq!(cookie.same_site.as_deref(), Some("Lax"));
}

#[test]
fn builds_cookie_headers() {
    let header = build_cookie_header(&[
        Cookie {
            name: String::from("session"),
            value: String::from("abc"),
        },
        Cookie {
            name: String::from("theme"),
            value: String::from("dark"),
        },
    ]);

    assert_eq!(header, "session=abc; theme=dark");
}

#[test]
fn looks_up_cookies() {
    assert_eq!(
        get_cookie("session=abc; theme=dark", "theme").as_deref(),
        Some("dark")
    );
    assert!(has_cookie("session=abc; theme=dark", "session"));
}

#[test]
fn reports_missing_cookies() {
    assert_eq!(get_cookie("session=abc", "theme"), None);
    assert!(!has_cookie("session=abc", "theme"));
}

#[test]
fn validates_cookie_names() {
    assert!(cookie_name_is_valid("session_id"));
    assert!(!cookie_name_is_valid("bad name"));
}

#[test]
fn handles_malformed_and_empty_input() {
    assert!(parse_cookie_header("").is_empty());
    assert_eq!(parse_set_cookie_basic("broken"), None);
    assert_eq!(parse_set_cookie_basic("bad name=value"), None);
}
