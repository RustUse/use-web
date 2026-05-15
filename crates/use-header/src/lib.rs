#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A simple header name-value pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub name: String,
    pub value: String,
}

/// Normalizes a header name to lowercase ASCII.
#[must_use]
pub fn normalize_header_name(input: &str) -> String {
    input.trim().to_ascii_lowercase()
}

/// Returns `true` when the input is a valid HTTP field name.
#[must_use]
pub fn is_valid_header_name(input: &str) -> bool {
    let trimmed = input.trim();
    !trimmed.is_empty() && trimmed.bytes().all(is_token_byte)
}

/// Parses a single `Name: Value` header line.
#[must_use]
pub fn parse_header_line(input: &str) -> Option<Header> {
    let (name, value) = input.split_once(':')?;
    if !is_valid_header_name(name) {
        return None;
    }

    Some(Header {
        name: normalize_header_name(name),
        value: value.trim().to_string(),
    })
}

/// Parses multiple header lines separated by newlines.
#[must_use]
pub fn parse_headers(input: &str) -> Vec<Header> {
    input
        .lines()
        .filter_map(|line| parse_header_line(line.trim_end_matches('\r')))
        .collect()
}

/// Returns the first matching header value, compared case-insensitively.
#[must_use]
pub fn get_header(headers: &[Header], name: &str) -> Option<String> {
    let normalized = normalize_header_name(name);

    headers
        .iter()
        .find(|header| normalize_header_name(&header.name) == normalized)
        .map(|header| header.value.clone())
}

/// Returns `true` when the header list contains the requested name.
#[must_use]
pub fn has_header(headers: &[Header], name: &str) -> bool {
    get_header(headers, name).is_some()
}

/// Sets or replaces a header value.
pub fn set_header(headers: &mut Vec<Header>, name: &str, value: &str) {
    if !is_valid_header_name(name) {
        return;
    }

    remove_header(headers, name);
    headers.push(Header {
        name: normalize_header_name(name),
        value: value.trim().to_string(),
    });
}

/// Removes every header whose name matches case-insensitively.
pub fn remove_header(headers: &mut Vec<Header>, name: &str) {
    let normalized = normalize_header_name(name);
    headers.retain(|header| normalize_header_name(&header.name) != normalized);
}

fn is_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}
