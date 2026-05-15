#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A cookie name-value pair from the `Cookie` header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

/// A lightweight view of a `Set-Cookie` header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCookie {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub max_age: Option<i64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
}

/// Parses a `Cookie` header into cookie pairs.
#[must_use]
pub fn parse_cookie_header(input: &str) -> Vec<Cookie> {
    input
        .split(';')
        .filter_map(|segment| {
            let trimmed = segment.trim();
            let (name, value) = trimmed.split_once('=')?;
            let name = name.trim();
            cookie_name_is_valid(name).then(|| Cookie {
                name: name.to_string(),
                value: value.trim().to_string(),
            })
        })
        .collect()
}

/// Parses a lightweight `Set-Cookie` header.
#[must_use]
pub fn parse_set_cookie_basic(input: &str) -> Option<SetCookie> {
    let mut parts = input.split(';');
    let first = parts.next()?.trim();
    let (name, value) = first.split_once('=')?;
    let name = name.trim();
    if !cookie_name_is_valid(name) {
        return None;
    }

    let mut cookie = SetCookie {
        name: name.to_string(),
        value: value.trim().to_string(),
        path: None,
        domain: None,
        max_age: None,
        secure: false,
        http_only: false,
        same_site: None,
    };

    for attribute in parts {
        let attribute = attribute.trim();
        if attribute.eq_ignore_ascii_case("secure") {
            cookie.secure = true;
            continue;
        }
        if attribute.eq_ignore_ascii_case("httponly") {
            cookie.http_only = true;
            continue;
        }

        if let Some((key, raw_value)) = attribute.split_once('=') {
            let key = key.trim();
            let value = raw_value.trim();
            if key.eq_ignore_ascii_case("path") {
                cookie.path = Some(value.to_string());
            } else if key.eq_ignore_ascii_case("domain") {
                cookie.domain = Some(value.to_string());
            } else if key.eq_ignore_ascii_case("max-age") {
                cookie.max_age = value.parse().ok();
            } else if key.eq_ignore_ascii_case("samesite") && !value.is_empty() {
                cookie.same_site = Some(value.to_string());
            }
        }
    }

    Some(cookie)
}

/// Builds a `Cookie` header string from cookie pairs.
#[must_use]
pub fn build_cookie_header(cookies: &[Cookie]) -> String {
    cookies
        .iter()
        .map(|cookie| format!("{}={}", cookie.name, cookie.value))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Returns the first cookie value with the requested name.
#[must_use]
pub fn get_cookie(input: &str, name: &str) -> Option<String> {
    parse_cookie_header(input)
        .into_iter()
        .find(|cookie| cookie.name == name)
        .map(|cookie| cookie.value)
}

/// Returns `true` when the header contains the requested cookie.
#[must_use]
pub fn has_cookie(input: &str, name: &str) -> bool {
    get_cookie(input, name).is_some()
}

/// Returns `true` when the cookie name uses a conservative HTTP token character set.
#[must_use]
pub fn cookie_name_is_valid(input: &str) -> bool {
    let trimmed = input.trim();
    !trimmed.is_empty() && trimmed.bytes().all(is_token_byte)
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
