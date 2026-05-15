#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Known HTTP methods plus a variant for valid extension methods.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Trace,
    Connect,
    Other(String),
}

/// Parses a method token into a standard variant or `Other`.
#[must_use]
pub fn parse_method(input: &str) -> Option<HttpMethod> {
    let normalized = normalize_method(input)?;

    Some(match normalized.as_str() {
        "GET" => HttpMethod::Get,
        "HEAD" => HttpMethod::Head,
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "PATCH" => HttpMethod::Patch,
        "DELETE" => HttpMethod::Delete,
        "OPTIONS" => HttpMethod::Options,
        "TRACE" => HttpMethod::Trace,
        "CONNECT" => HttpMethod::Connect,
        _ => HttpMethod::Other(normalized),
    })
}

/// Normalizes a valid HTTP method token to uppercase.
#[must_use]
pub fn normalize_method(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.chars().any(char::is_whitespace) {
        return None;
    }

    if trimmed.bytes().all(is_token_byte) {
        Some(trimmed.to_ascii_uppercase())
    } else {
        None
    }
}

/// Returns `true` when the token is one of the standard request methods.
#[must_use]
pub fn is_standard_method(input: &str) -> bool {
    matches!(
        normalize_method(input).as_deref(),
        Some(
            "GET" | "HEAD" | "POST" | "PUT" | "PATCH" | "DELETE" | "OPTIONS" | "TRACE" | "CONNECT"
        )
    )
}

/// Returns `true` when the method is defined as safe.
#[must_use]
pub fn is_safe_method(input: &str) -> bool {
    matches!(
        normalize_method(input).as_deref(),
        Some("GET" | "HEAD" | "OPTIONS" | "TRACE")
    )
}

/// Returns `true` when the method is defined as idempotent.
#[must_use]
pub fn is_idempotent_method(input: &str) -> bool {
    matches!(
        normalize_method(input).as_deref(),
        Some("GET" | "HEAD" | "OPTIONS" | "TRACE" | "PUT" | "DELETE")
    )
}

/// Returns `true` when the method can reasonably carry a body.
#[must_use]
pub fn method_allows_body(input: &str) -> bool {
    match parse_method(input) {
        Some(HttpMethod::Get | HttpMethod::Head | HttpMethod::Trace) | None => false,
        Some(_) => true,
    }
}

/// Returns `true` when the method usually implies a request body.
#[must_use]
pub fn method_expects_body(input: &str) -> bool {
    matches!(
        normalize_method(input).as_deref(),
        Some("POST" | "PUT" | "PATCH")
    )
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
