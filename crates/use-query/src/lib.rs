#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A parsed query parameter with an optional value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryParam {
    pub key: String,
    pub value: Option<String>,
}

/// Parses a query string into ordered key-value pairs.
#[must_use]
pub fn parse_query(input: &str) -> Vec<QueryParam> {
    let query = normalize_query_input(input);
    if query.is_empty() {
        return Vec::new();
    }

    query
        .split('&')
        .filter(|segment| !segment.is_empty())
        .map(|segment| match segment.split_once('=') {
            Some((key, value)) => QueryParam {
                key: key.to_string(),
                value: Some(value.to_string()),
            },
            None => QueryParam {
                key: segment.to_string(),
                value: None,
            },
        })
        .collect()
}

/// Builds a query string from ordered parameters.
#[must_use]
pub fn build_query(params: &[QueryParam]) -> String {
    params
        .iter()
        .map(|param| match &param.value {
            Some(value) => format!("{}={value}", param.key),
            None => param.key.clone(),
        })
        .collect::<Vec<_>>()
        .join("&")
}

/// Returns the first value for the requested key.
#[must_use]
pub fn get_query_param(input: &str, key: &str) -> Option<String> {
    parse_query(input)
        .into_iter()
        .find(|param| param.key == key)
        .and_then(|param| param.value)
}

/// Returns `true` when the query contains the requested key.
#[must_use]
pub fn has_query_param(input: &str, key: &str) -> bool {
    parse_query(input).into_iter().any(|param| param.key == key)
}

/// Removes every matching key from the query string.
#[must_use]
pub fn remove_query_param(input: &str, key: &str) -> String {
    let filtered = parse_query(input)
        .into_iter()
        .filter(|param| param.key != key)
        .collect::<Vec<_>>();

    build_query(&filtered)
}

/// Appends a key-value pair to the query string.
#[must_use]
pub fn append_query_param(input: &str, key: &str, value: &str) -> String {
    let mut params = parse_query(input);
    params.push(QueryParam {
        key: key.to_string(),
        value: Some(value.to_string()),
    });
    build_query(&params)
}

fn normalize_query_input(input: &str) -> &str {
    let trimmed = input.trim();
    let before_fragment = trimmed
        .split_once('#')
        .map_or(trimmed, |(before, _)| before);

    if let Some((_, query)) = before_fragment.split_once('?') {
        query
    } else {
        before_fragment.strip_prefix('?').unwrap_or(before_fragment)
    }
}
