#[cfg(feature = "uri")]
#[test]
fn facade_reexports_uri_items() {
    assert!(use_web::looks_like_uri("https://example.com/docs"));
    assert_eq!(
        use_web::uri::extract_scheme("MailTo:user@example.com").as_deref(),
        Some("mailto")
    );
}

#[cfg(all(feature = "url", feature = "origin", feature = "route"))]
#[test]
fn facade_supports_url_origin_route_workflow() {
    assert_eq!(
        use_web::extract_host("https://example.com/docs").as_deref(),
        Some("example.com")
    );
    assert!(use_web::same_origin(
        "https://example.com/docs",
        "https://example.com/api"
    ));
    assert!(use_web::route::matches_route_pattern(
        "/docs/:page",
        "/docs/api"
    ));
    assert_eq!(use_web::route_depth("/docs/api"), 2);
}

#[cfg(all(feature = "http", feature = "status"))]
#[test]
fn facade_supports_http_and_status_reexports() {
    let request = use_web::parse_request_line("GET /docs HTTP/1.1").unwrap();

    assert_eq!(request.method, "GET");
    assert_eq!(use_web::reason_phrase(404), Some("Not Found"));
}

#[cfg(all(
    feature = "http",
    feature = "method",
    feature = "header",
    feature = "status",
    feature = "cookie",
    feature = "mime",
    feature = "uri",
    feature = "url",
    feature = "query",
    feature = "origin",
    feature = "route",
    feature = "html",
    feature = "css"
))]
#[test]
fn facade_exposes_all_namespace_features() {
    use use_web::{
        cookie as _, css as _, header as _, html as _, http as _, method as _, mime as _,
        origin as _, query as _, route as _, status as _, uri as _, url as _,
    };

    let _ = use_web::parse_method("GET");
    let _ = use_web::split_css_declaration("color: red;");
}
