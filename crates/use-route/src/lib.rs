#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A normalized route segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteSegment {
    pub value: String,
}

/// A captured route parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteParam {
    pub name: String,
    pub value: String,
}

/// Normalizes repeated slashes and removes trailing slashes except for the root route.
#[must_use]
pub fn normalize_route(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::from("/");
    }

    let absolute = trimmed.starts_with('/');
    let segments = trimmed
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if segments.is_empty() {
        return String::from("/");
    }

    let joined = segments.join("/");
    if absolute {
        format!("/{joined}")
    } else {
        joined
    }
}

/// Joins two route fragments with exactly one slash.
#[must_use]
pub fn join_routes(a: &str, b: &str) -> String {
    let absolute = a.trim().starts_with('/');
    let segments = a
        .split('/')
        .chain(b.split('/'))
        .filter(|segment| !segment.trim().is_empty())
        .collect::<Vec<_>>();

    if segments.is_empty() {
        return if absolute {
            String::from("/")
        } else {
            String::new()
        };
    }

    let joined = segments.join("/");
    if absolute {
        format!("/{joined}")
    } else {
        joined
    }
}

/// Splits a route into normalized segments.
#[must_use]
pub fn split_route(input: &str) -> Vec<RouteSegment> {
    let normalized = normalize_route(input);
    normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(|segment| RouteSegment {
            value: segment.to_string(),
        })
        .collect()
}

/// Returns `true` when the route is absolute.
#[must_use]
pub fn is_absolute_route(input: &str) -> bool {
    input.trim().starts_with('/')
}

/// Returns the number of non-empty route segments.
#[must_use]
pub fn route_depth(input: &str) -> usize {
    split_route(input).len()
}

/// Returns `true` when a simple `:param` pattern matches the path.
#[must_use]
pub fn matches_route_pattern(pattern: &str, path: &str) -> bool {
    let pattern_segments = split_route(pattern);
    let path_segments = split_route(path);
    if pattern_segments.len() != path_segments.len() {
        return false;
    }

    pattern_segments
        .iter()
        .zip(path_segments.iter())
        .all(|(pattern_segment, path_segment)| {
            pattern_segment.value.starts_with(':') || pattern_segment.value == path_segment.value
        })
}

/// Extracts named route parameters when a pattern matches.
#[must_use]
pub fn extract_route_params(pattern: &str, path: &str) -> Vec<RouteParam> {
    if !matches_route_pattern(pattern, path) {
        return Vec::new();
    }

    split_route(pattern)
        .into_iter()
        .zip(split_route(path))
        .filter_map(|(pattern_segment, path_segment)| {
            pattern_segment
                .value
                .strip_prefix(':')
                .filter(|name| !name.is_empty())
                .map(|name| RouteParam {
                    name: name.to_string(),
                    value: path_segment.value,
                })
        })
        .collect()
}
