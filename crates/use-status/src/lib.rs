#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A common HTTP status code and its standard reason phrase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HttpStatus {
    pub code: u16,
    pub reason: &'static str,
}

/// A broad HTTP status code class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusClass {
    Informational,
    Success,
    Redirection,
    ClientError,
    ServerError,
}

/// Returns the standard reason phrase for a common status code.
#[must_use]
pub const fn reason_phrase(code: u16) -> Option<&'static str> {
    match code {
        100 => Some("Continue"),
        101 => Some("Switching Protocols"),
        102 => Some("Processing"),
        103 => Some("Early Hints"),
        200 => Some("OK"),
        201 => Some("Created"),
        202 => Some("Accepted"),
        204 => Some("No Content"),
        206 => Some("Partial Content"),
        300 => Some("Multiple Choices"),
        301 => Some("Moved Permanently"),
        302 => Some("Found"),
        303 => Some("See Other"),
        304 => Some("Not Modified"),
        307 => Some("Temporary Redirect"),
        308 => Some("Permanent Redirect"),
        400 => Some("Bad Request"),
        401 => Some("Unauthorized"),
        403 => Some("Forbidden"),
        404 => Some("Not Found"),
        405 => Some("Method Not Allowed"),
        409 => Some("Conflict"),
        410 => Some("Gone"),
        415 => Some("Unsupported Media Type"),
        422 => Some("Unprocessable Content"),
        429 => Some("Too Many Requests"),
        500 => Some("Internal Server Error"),
        501 => Some("Not Implemented"),
        502 => Some("Bad Gateway"),
        503 => Some("Service Unavailable"),
        504 => Some("Gateway Timeout"),
        _ => None,
    }
}

/// Returns the broad HTTP status class for a code in the standard range.
#[must_use]
pub const fn status_class(code: u16) -> Option<StatusClass> {
    match code {
        100..=199 => Some(StatusClass::Informational),
        200..=299 => Some(StatusClass::Success),
        300..=399 => Some(StatusClass::Redirection),
        400..=499 => Some(StatusClass::ClientError),
        500..=599 => Some(StatusClass::ServerError),
        _ => None,
    }
}

/// Returns `true` when the code is informational.
#[must_use]
pub const fn is_informational(code: u16) -> bool {
    matches!(status_class(code), Some(StatusClass::Informational))
}

/// Returns `true` when the code is successful.
#[must_use]
pub const fn is_success(code: u16) -> bool {
    matches!(status_class(code), Some(StatusClass::Success))
}

/// Returns `true` when the code is a redirect.
#[must_use]
pub const fn is_redirect(code: u16) -> bool {
    matches!(status_class(code), Some(StatusClass::Redirection))
}

/// Returns `true` when the code is a client error.
#[must_use]
pub const fn is_client_error(code: u16) -> bool {
    matches!(status_class(code), Some(StatusClass::ClientError))
}

/// Returns `true` when the code is a server error.
#[must_use]
pub const fn is_server_error(code: u16) -> bool {
    matches!(status_class(code), Some(StatusClass::ServerError))
}

/// Returns `true` when the code is a client or server error.
#[must_use]
pub const fn is_error(code: u16) -> bool {
    is_client_error(code) || is_server_error(code)
}

/// Returns `true` for codes that are cacheable by default in common HTTP semantics.
#[must_use]
pub const fn is_cacheable_by_default(code: u16) -> bool {
    matches!(
        code,
        200 | 203 | 204 | 206 | 300 | 301 | 404 | 405 | 410 | 414 | 501
    )
}
