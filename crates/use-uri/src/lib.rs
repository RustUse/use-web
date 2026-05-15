#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Basic URI parts extracted with lightweight string splitting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UriParts {
    /// The URI scheme when a valid scheme prefix exists.
    pub scheme: Option<String>,
    /// The authority component when the URI uses `//authority` syntax.
    pub authority: Option<String>,
    /// The path component, which may be empty.
    pub path: String,
    /// The query without the leading `?` when present.
    pub query: Option<String>,
    /// The fragment without the leading `#` when present.
    pub fragment: Option<String>,
}

/// Returns `true` when the input starts with a valid URI scheme or authority marker.
#[must_use]
pub fn looks_like_uri(input: &str) -> bool {
    let trimmed = input.trim();
    !trimmed.is_empty() && (has_scheme(trimmed) || trimmed.starts_with("//"))
}

/// Parses URI-like input with lightweight splitting and graceful fallback behavior.
#[must_use]
pub fn parse_uri_basic(input: &str) -> UriParts {
    let trimmed = input.trim();
    let scheme = extract_scheme(trimmed);
    let mut remainder = if let Some(scheme) = scheme.as_deref() {
        &trimmed[scheme.len() + 1..]
    } else {
        trimmed
    };

    let authority = if let Some(after_slashes) = remainder.strip_prefix("//") {
        let end = after_slashes
            .find(['/', '?', '#'])
            .unwrap_or(after_slashes.len());
        remainder = &after_slashes[end..];
        let value = &after_slashes[..end];
        (!value.is_empty()).then(|| value.to_string())
    } else {
        None
    };

    let fragment_index = remainder.find('#');
    let without_fragment = &remainder[..fragment_index.unwrap_or(remainder.len())];
    let query_index = without_fragment.find('?');

    let path = without_fragment[..query_index.unwrap_or(without_fragment.len())].to_string();
    let query = query_index.map(|index| without_fragment[index + 1..].to_string());
    let fragment = fragment_index.map(|index| remainder[index + 1..].to_string());

    UriParts {
        scheme,
        authority,
        path,
        query,
        fragment,
    }
}

/// Returns `true` when the input begins with a valid URI scheme.
#[must_use]
pub fn has_scheme(input: &str) -> bool {
    extract_scheme(input).is_some()
}

/// Extracts a lowercase URI scheme when the input starts with one.
#[must_use]
pub fn extract_scheme(input: &str) -> Option<String> {
    let trimmed = input.trim();
    let colon_index = trimmed.find(':')?;
    let candidate = &trimmed[..colon_index];

    if candidate.is_empty() {
        return None;
    }

    let mut chars = candidate.chars();
    let first = chars.next()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }

    if chars
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.'))
    {
        Some(candidate.to_ascii_lowercase())
    } else {
        None
    }
}

/// Extracts the fragment without the leading `#` when present.
#[must_use]
pub fn extract_fragment(input: &str) -> Option<String> {
    let fragment_index = input.find('#')?;
    Some(input[fragment_index + 1..].to_string())
}

/// Returns the input without the fragment portion.
#[must_use]
pub fn strip_fragment(input: &str) -> &str {
    match input.find('#') {
        Some(index) => &input[..index],
        None => input,
    }
}

/// Extracts the query without the leading `?` and excluding any fragment.
#[must_use]
pub fn extract_query(input: &str) -> Option<String> {
    let without_fragment = strip_fragment(input);
    let query_index = without_fragment.find('?')?;
    Some(without_fragment[query_index + 1..].to_string())
}

/// Returns the input without the query portion.
#[must_use]
pub fn strip_query(input: &str) -> &str {
    match input.find('?') {
        Some(index) => &input[..index],
        None => input,
    }
}
