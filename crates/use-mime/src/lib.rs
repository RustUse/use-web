#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A lightweight MIME type split into type, subtype, and optional suffix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MimeType {
    pub type_: String,
    pub subtype: String,
    pub suffix: Option<String>,
}

/// Parses a MIME type essence into simple parts.
#[must_use]
pub fn parse_mime(input: &str) -> Option<MimeType> {
    let essence = input.trim().split(';').next()?.trim().to_ascii_lowercase();
    let (type_, subtype_with_suffix) = essence.split_once('/')?;
    if type_.is_empty() || subtype_with_suffix.is_empty() {
        return None;
    }
    if !is_token(type_) || !is_token(subtype_with_suffix) {
        return None;
    }

    let (subtype, suffix) = match subtype_with_suffix.rsplit_once('+') {
        Some((subtype, suffix)) if !subtype.is_empty() && !suffix.is_empty() => {
            (subtype.to_string(), Some(suffix.to_string()))
        }
        _ => (subtype_with_suffix.to_string(), None),
    };

    Some(MimeType {
        type_: type_.to_string(),
        subtype,
        suffix,
    })
}

/// Returns `true` when the input parses as a MIME type.
#[must_use]
pub fn looks_like_mime(input: &str) -> bool {
    parse_mime(input).is_some()
}

/// Returns a common MIME type for the requested extension.
#[must_use]
pub fn mime_from_extension(extension: &str) -> Option<&'static str> {
    match extension
        .trim()
        .trim_start_matches('.')
        .to_ascii_lowercase()
        .as_str()
    {
        "html" | "htm" => Some("text/html"),
        "css" => Some("text/css"),
        "js" | "mjs" => Some("application/javascript"),
        "json" => Some("application/json"),
        "xml" => Some("application/xml"),
        "txt" => Some("text/plain"),
        "md" => Some("text/markdown"),
        "csv" => Some("text/csv"),
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        "svg" => Some("image/svg+xml"),
        "webp" => Some("image/webp"),
        "ico" => Some("image/x-icon"),
        "pdf" => Some("application/pdf"),
        "wasm" => Some("application/wasm"),
        "zip" => Some("application/zip"),
        _ => None,
    }
}

/// Returns a common extension for the requested MIME type.
#[must_use]
pub fn extension_from_mime(mime: &str) -> Option<&'static str> {
    match mime
        .trim()
        .split(';')
        .next()?
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "text/html" => Some("html"),
        "text/css" => Some("css"),
        "application/javascript" | "text/javascript" => Some("js"),
        "application/json" => Some("json"),
        "application/xml" | "text/xml" => Some("xml"),
        "text/plain" => Some("txt"),
        "text/markdown" => Some("md"),
        "text/csv" => Some("csv"),
        "image/png" => Some("png"),
        "image/jpeg" => Some("jpg"),
        "image/gif" => Some("gif"),
        "image/svg+xml" => Some("svg"),
        "image/webp" => Some("webp"),
        "image/x-icon" => Some("ico"),
        "application/pdf" => Some("pdf"),
        "application/wasm" => Some("wasm"),
        "application/zip" => Some("zip"),
        _ => None,
    }
}

/// Returns `true` when the MIME type is text-based.
#[must_use]
pub fn is_text_mime(mime: &str) -> bool {
    matches!(parse_mime(mime), Some(MimeType { type_, .. }) if type_ == "text")
}

/// Returns `true` when the MIME type is image-based.
#[must_use]
pub fn is_image_mime(mime: &str) -> bool {
    matches!(parse_mime(mime), Some(MimeType { type_, .. }) if type_ == "image")
}

/// Returns `true` when the MIME type is JSON or a `+json` subtype.
#[must_use]
pub fn is_json_mime(mime: &str) -> bool {
    matches!(parse_mime(mime), Some(MimeType { subtype, suffix, .. }) if subtype == "json" || suffix.as_deref() == Some("json"))
}

/// Returns `true` when the MIME type is HTML.
#[must_use]
pub fn is_html_mime(mime: &str) -> bool {
    matches!(parse_mime(mime), Some(MimeType { type_, subtype, .. }) if type_ == "text" && subtype == "html")
}

/// Returns `true` when the MIME type is XML or a `+xml` subtype.
#[must_use]
pub fn is_xml_mime(mime: &str) -> bool {
    matches!(parse_mime(mime), Some(MimeType { subtype, suffix, .. }) if subtype == "xml" || suffix.as_deref() == Some("xml"))
}

/// Returns `true` when the MIME type is CSS.
#[must_use]
pub fn is_css_mime(mime: &str) -> bool {
    matches!(parse_mime(mime), Some(MimeType { type_, subtype, .. }) if type_ == "text" && subtype == "css")
}

fn is_token(input: &str) -> bool {
    input.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'!' | b'#' | b'$' | b'&' | b'^' | b'_' | b'.' | b'+' | b'-'
            )
    })
}
