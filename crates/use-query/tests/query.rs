use use_query::{
    QueryParam, append_query_param, build_query, get_query_param, has_query_param, parse_query,
    remove_query_param,
};

#[test]
fn parses_query_strings() {
    let params = parse_query("page=2&lang=en");

    assert_eq!(params.len(), 2);
    assert_eq!(params[0].key, "page");
    assert_eq!(params[0].value.as_deref(), Some("2"));
}

#[test]
fn builds_query_strings() {
    let query = build_query(&[
        QueryParam {
            key: String::from("page"),
            value: Some(String::from("2")),
        },
        QueryParam {
            key: String::from("draft"),
            value: None,
        },
    ]);

    assert_eq!(query, "page=2&draft");
}

#[test]
fn supports_repeated_keys() {
    let params = parse_query("tag=rust&tag=web");

    assert_eq!(params.len(), 2);
    assert_eq!(params[1].value.as_deref(), Some("web"));
}

#[test]
fn supports_empty_and_missing_values() {
    let params = parse_query("empty=&draft");

    assert_eq!(params[0].value.as_deref(), Some(""));
    assert_eq!(params[1].value, None);
}

#[test]
fn looks_up_params() {
    assert_eq!(
        get_query_param("page=2&lang=en", "lang").as_deref(),
        Some("en")
    );
    assert!(has_query_param("page=2&lang=en", "page"));
}

#[test]
fn removes_and_appends_params() {
    assert_eq!(remove_query_param("page=2&lang=en", "page"), "lang=en");
    assert_eq!(append_query_param("page=2", "lang", "en"), "page=2&lang=en");
}

#[test]
fn handles_empty_input() {
    assert!(parse_query("").is_empty());
    assert_eq!(append_query_param("", "page", "2"), "page=2");
}
