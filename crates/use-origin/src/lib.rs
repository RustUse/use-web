#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A lightweight web origin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Origin {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
}

/// Parses a strict origin string without path, query, or fragment content.
#[must_use]
pub fn parse_origin(input: &str) -> Option<Origin> {
    let trimmed = input.trim();
    let origin = extract_origin(trimmed)?;
    let normalized = trimmed.trim_end_matches('/');
    normalized
        .eq_ignore_ascii_case(&format_origin(&origin))
        .then_some(origin)
}

/// Extracts the origin portion from a URL or origin-like string.
#[must_use]
pub fn extract_origin(input: &str) -> Option<Origin> {
    let trimmed = input.trim();
    let scheme = extract_scheme(trimmed)?;
    let remainder = &trimmed[scheme.len() + 1..];
    let authority_input = remainder.strip_prefix("//")?;
    let authority_end = authority_input
        .find(['/', '?', '#'])
        .unwrap_or(authority_input.len());
    let authority = &authority_input[..authority_end];
    let (host, port) = parse_authority(authority)?;

    Some(Origin { scheme, host, port })
}

/// Returns `true` when both inputs resolve to the same origin.
#[must_use]
pub fn same_origin(a: &str, b: &str) -> bool {
    let Some(left) = extract_origin(a) else {
        return false;
    };
    let Some(right) = extract_origin(b) else {
        return false;
    };

    let left_port = left.port.or_else(|| default_port_for_scheme(&left.scheme));
    let right_port = right
        .port
        .or_else(|| default_port_for_scheme(&right.scheme));

    left.scheme.eq_ignore_ascii_case(&right.scheme)
        && left.host.eq_ignore_ascii_case(&right.host)
        && left_port == right_port
}

/// Formats an origin as `scheme://host[:port]`.
#[must_use]
pub fn format_origin(origin: &Origin) -> String {
    match origin.port {
        Some(port) => format!("{}://{}:{port}", origin.scheme, origin.host),
        None => format!("{}://{}", origin.scheme, origin.host),
    }
}

/// Returns `true` for secure-origin schemes.
#[must_use]
pub fn is_secure_origin(input: &str) -> bool {
    matches!(extract_origin(input), Some(Origin { scheme, .. }) if matches!(scheme.as_str(), "https" | "wss"))
}

/// Returns the conventional default port for a scheme.
#[must_use]
pub fn default_port_for_scheme(scheme: &str) -> Option<u16> {
    match scheme.trim().to_ascii_lowercase().as_str() {
        "http" | "ws" => Some(80),
        "https" | "wss" => Some(443),
        _ => None,
    }
}

fn extract_scheme(input: &str) -> Option<String> {
    let colon_index = input.find(':')?;
    let candidate = &input[..colon_index];
    if candidate.is_empty() {
        return None;
    }
    let mut characters = candidate.chars();
    let first = characters.next()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }
    if characters
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.'))
    {
        Some(candidate.to_ascii_lowercase())
    } else {
        None
    }
}

fn parse_authority(authority: &str) -> Option<(String, Option<u16>)> {
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, tail)| tail);
    if host_port.is_empty() {
        return None;
    }

    if let Some(after_bracket) = host_port.strip_prefix('[') {
        let end = after_bracket.find(']')?;
        let host = after_bracket[..end].to_string();
        let tail = &after_bracket[end + 1..];
        let port = tail.strip_prefix(':').and_then(|value| value.parse().ok());
        return Some((host, port));
    }

    if let Some((host, port_text)) = host_port.rsplit_once(':') {
        if !host.is_empty()
            && !port_text.is_empty()
            && port_text
                .chars()
                .all(|character| character.is_ascii_digit())
        {
            return Some((host.to_string(), port_text.parse().ok()));
        }
    }

    Some((host_port.to_string(), None))
}
