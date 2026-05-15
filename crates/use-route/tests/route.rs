use use_route::{
    extract_route_params, is_absolute_route, join_routes, matches_route_pattern, normalize_route,
    route_depth, split_route,
};

#[test]
fn normalizes_routes() {
    assert_eq!(normalize_route("//users//42/"), "/users/42");
}

#[test]
fn joins_routes() {
    assert_eq!(join_routes("/api", "users/42"), "/api/users/42");
    assert_eq!(join_routes("api", "/users/42"), "api/users/42");
}

#[test]
fn splits_routes() {
    let segments = split_route("/users/42");
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0].value, "users");
}

#[test]
fn reports_depth_and_absolute_status() {
    assert!(is_absolute_route("/users/42"));
    assert_eq!(route_depth("/posts/rust/comments/1"), 4);
}

#[test]
fn matches_route_patterns() {
    assert!(matches_route_pattern("/users/:id", "/users/42"));
    assert!(matches_route_pattern(
        "/posts/:slug/comments/:comment_id",
        "/posts/rust/comments/1"
    ));
    assert!(!matches_route_pattern("/users/:id", "/posts/42"));
}

#[test]
fn extracts_route_params() {
    let params = extract_route_params(
        "/posts/:slug/comments/:comment_id",
        "/posts/rust/comments/1",
    );

    assert_eq!(params.len(), 2);
    assert_eq!(params[0].name, "slug");
    assert_eq!(params[0].value, "rust");
    assert_eq!(params[1].name, "comment_id");
    assert_eq!(params[1].value, "1");
}

#[test]
fn handles_empty_input() {
    assert_eq!(normalize_route(""), "/");
    assert!(split_route("").is_empty());
}
