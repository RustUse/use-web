#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Lightweight URL parts extracted with simple string splitting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlParts {
    pub scheme: String,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

/// Returns `true` when the input looks like a parsable absolute URL.
#[must_use]
pub fn looks_like_url(input: &str) -> bool {
    parse_url_basic(input).is_some()
}

/// Returns `true` when the input is an HTTP URL.
#[must_use]
pub fn is_http_url(input: &str) -> bool {
    matches!(parse_url_basic(input), Some(UrlParts { scheme, .. }) if scheme == "http")
}

/// Returns `true` when the input is an HTTPS URL.
#[must_use]
pub fn is_https_url(input: &str) -> bool {
    matches!(parse_url_basic(input), Some(UrlParts { scheme, .. }) if scheme == "https")
}

/// Parses a URL with lightweight string splitting.
#[must_use]
pub fn parse_url_basic(input: &str) -> Option<UrlParts> {
    let trimmed = input.trim();
    let scheme = extract_scheme(trimmed)?;
    let remainder = &trimmed[scheme.len() + 1..];
    let authority_input = remainder.strip_prefix("//")?;
    let authority_end = authority_input
        .find(['/', '?', '#'])
        .unwrap_or(authority_input.len());
    let authority = &authority_input[..authority_end];
    if authority.is_empty() {
        return None;
    }

    let (host, port) = parse_authority(authority);
    let suffix = &authority_input[authority_end..];
    let fragment_index = suffix.find('#');
    let without_fragment = &suffix[..fragment_index.unwrap_or(suffix.len())];
    let query_index = without_fragment.find('?');
    let path = without_fragment[..query_index.unwrap_or(without_fragment.len())].to_string();
    let path = if path.is_empty() {
        String::from("/")
    } else {
        path
    };

    Some(UrlParts {
        scheme,
        host,
        port,
        path,
        query: query_index.map(|index| without_fragment[index + 1..].to_string()),
        fragment: fragment_index.map(|index| suffix[index + 1..].to_string()),
    })
}

/// Extracts the URL host when available.
#[must_use]
pub fn extract_host(input: &str) -> Option<String> {
    parse_url_basic(input).and_then(|parts| parts.host)
}

/// Extracts the URL port when available.
#[must_use]
pub fn extract_port(input: &str) -> Option<u16> {
    parse_url_basic(input).and_then(|parts| parts.port)
}

/// Extracts the URL path when available.
#[must_use]
pub fn extract_path(input: &str) -> Option<String> {
    parse_url_basic(input).map(|parts| parts.path)
}

/// Ensures that the URL or path ends with a trailing slash before any query or fragment.
#[must_use]
pub fn ensure_trailing_slash(input: &str) -> String {
    let (base, suffix) = split_suffix(input.trim());
    if base.is_empty() {
        return format!("/{suffix}");
    }
    if base.ends_with('/') {
        input.trim().to_string()
    } else {
        format!("{base}/{suffix}")
    }
}

/// Removes trailing slashes before any query or fragment.
#[must_use]
pub fn strip_trailing_slash(input: &str) -> String {
    let (base, suffix) = split_suffix(input.trim());
    let trimmed = if base == "/" {
        "/"
    } else {
        base.trim_end_matches('/')
    };

    format!("{trimmed}{suffix}")
}

/// Removes the fragment portion from a URL.
#[must_use]
pub fn remove_fragment(input: &str) -> String {
    match input.find('#') {
        Some(index) => input[..index].to_string(),
        None => input.to_string(),
    }
}

/// Removes the query portion and preserves any fragment.
#[must_use]
pub fn remove_query(input: &str) -> String {
    match input.find('?') {
        Some(query_index) => {
            let fragment = input[query_index..]
                .find('#')
                .map(|offset| &input[query_index + offset..])
                .unwrap_or("");
            format!("{}{fragment}", &input[..query_index])
        },
        None => input.to_string(),
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

fn parse_authority(authority: &str) -> (Option<String>, Option<u16>) {
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, tail)| tail);

    if let Some(after_bracket) = host_port.strip_prefix('[') {
        if let Some(end) = after_bracket.find(']') {
            let host = &after_bracket[..end];
            let tail = &after_bracket[end + 1..];
            let port = tail.strip_prefix(':').and_then(|value| value.parse().ok());
            return ((!host.is_empty()).then(|| host.to_string()), port);
        }
        return (None, None);
    }

    if let Some((host, port_text)) = host_port.rsplit_once(':') {
        if !host.is_empty()
            && !port_text.is_empty()
            && port_text
                .chars()
                .all(|character| character.is_ascii_digit())
        {
            return (Some(host.to_string()), port_text.parse().ok());
        }
    }

    ((!host_port.is_empty()).then(|| host_port.to_string()), None)
}

fn split_suffix(input: &str) -> (&str, &str) {
    let index = input.find(['?', '#']).unwrap_or(input.len());
    (&input[..index], &input[index..])
}
