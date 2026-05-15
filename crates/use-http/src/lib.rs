#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A small set of HTTP versions commonly encountered in textual protocol surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http2,
    Http3,
    Unknown,
}

/// A parsed HTTP request line.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HttpRequestLine {
    pub method: String,
    pub target: String,
    pub version: HttpVersion,
}

/// A parsed HTTP status line.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HttpStatusLine {
    pub version: HttpVersion,
    pub code: u16,
    pub reason: Option<String>,
}

/// Returns `true` when the input looks like a valid HTTP request line.
#[must_use]
pub fn looks_like_http_request_line(input: &str) -> bool {
    parse_request_line(input).is_some()
}

/// Returns `true` when the input looks like a valid HTTP status line.
#[must_use]
pub fn looks_like_http_status_line(input: &str) -> bool {
    parse_status_line(input).is_some()
}

/// Parses a textual HTTP version token.
#[must_use]
pub fn parse_http_version(input: &str) -> HttpVersion {
    match input.trim().to_ascii_uppercase().as_str() {
        "HTTP/1.0" => HttpVersion::Http10,
        "HTTP/1.1" => HttpVersion::Http11,
        "HTTP/2" | "HTTP/2.0" => HttpVersion::Http2,
        "HTTP/3" | "HTTP/3.0" => HttpVersion::Http3,
        _ => HttpVersion::Unknown,
    }
}

/// Formats an HTTP version token.
#[must_use]
pub const fn format_http_version(version: HttpVersion) -> &'static str {
    match version {
        HttpVersion::Http10 => "HTTP/1.0",
        HttpVersion::Http11 => "HTTP/1.1",
        HttpVersion::Http2 => "HTTP/2",
        HttpVersion::Http3 => "HTTP/3",
        HttpVersion::Unknown => "HTTP/?",
    }
}

/// Parses a request line in the form `METHOD target HTTP/x`.
#[must_use]
pub fn parse_request_line(input: &str) -> Option<HttpRequestLine> {
    let trimmed = input.trim();
    let (method, remainder) = split_once_whitespace(trimmed)?;
    let (target, version_text) = split_once_whitespace(remainder)?;
    if version_text.chars().any(char::is_whitespace) {
        return None;
    }

    let version = parse_http_version(version_text);
    if matches!(version, HttpVersion::Unknown) {
        return None;
    }

    Some(HttpRequestLine {
        method: method.to_string(),
        target: target.to_string(),
        version,
    })
}

/// Parses a status line in the form `HTTP/x 200 Reason`.
#[must_use]
pub fn parse_status_line(input: &str) -> Option<HttpStatusLine> {
    let trimmed = input.trim();
    let (version_text, remainder) = split_once_whitespace(trimmed)?;
    let version = parse_http_version(version_text);
    if matches!(version, HttpVersion::Unknown) {
        return None;
    }

    let (code_text, reason_text) = match split_once_whitespace(remainder) {
        Some((code, tail)) => (code, Some(tail.trim())),
        None => (remainder, None),
    };

    if code_text.len() != 3
        || !code_text
            .chars()
            .all(|character| character.is_ascii_digit())
    {
        return None;
    }

    Some(HttpStatusLine {
        version,
        code: code_text.parse().ok()?,
        reason: reason_text
            .filter(|reason| !reason.is_empty())
            .map(ToString::to_string),
    })
}

/// Returns `true` when the input is a recognized textual HTTP version.
#[must_use]
pub fn is_http_version(input: &str) -> bool {
    !matches!(parse_http_version(input), HttpVersion::Unknown)
}

/// Returns `true` when the target is in origin-form, such as `/items?limit=10`.
#[must_use]
pub fn is_request_target_origin_form(input: &str) -> bool {
    let trimmed = input.trim();
    trimmed.starts_with('/') && !trimmed.starts_with("//")
}

/// Returns `true` when the target is in absolute-form, such as `https://example.com/`.
#[must_use]
pub fn is_request_target_absolute_form(input: &str) -> bool {
    let trimmed = input.trim();
    if let Some(index) = trimmed.find("://") {
        let scheme = &trimmed[..index];
        !scheme.is_empty()
            && scheme.starts_with(|character: char| character.is_ascii_alphabetic())
            && scheme.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
            })
    } else {
        false
    }
}

/// Returns `true` when the target is in authority-form, such as `example.com:443`.
#[must_use]
pub fn is_request_target_authority_form(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty()
        || trimmed.contains(['/', '?', '#'])
        || trimmed.chars().any(char::is_whitespace)
    {
        return false;
    }

    if let Some(rest) = trimmed.strip_prefix('[') {
        if let Some(end) = rest.find(']') {
            let tail = &rest[end + 1..];
            return matches!(tail.strip_prefix(':'), Some(port) if !port.is_empty() && port.chars().all(|character| character.is_ascii_digit()));
        }
        return false;
    }

    matches!(trimmed.rsplit_once(':'), Some((host, port)) if !host.is_empty() && !port.is_empty() && port.chars().all(|character| character.is_ascii_digit()))
}

/// Returns `true` when the target is the asterisk-form `*`.
#[must_use]
pub fn is_request_target_asterisk_form(input: &str) -> bool {
    input.trim() == "*"
}

fn split_once_whitespace(input: &str) -> Option<(&str, &str)> {
    let index = input
        .char_indices()
        .find(|(_, character)| character.is_whitespace())?
        .0;
    let head = &input[..index];
    let tail = input[index..].trim_start();
    Some((head, tail))
}
