#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A lightweight HTML attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlAttribute {
    pub name: String,
    pub value: Option<String>,
}

/// A lightweight HTML element view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlElement {
    pub name: String,
    pub attributes: Vec<HtmlAttribute>,
}

/// A lightweight link extracted from an anchor tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlLink {
    pub text: String,
    pub href: String,
}

/// A lightweight heading extracted from `h1` through `h6`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlHeading {
    pub level: u8,
    pub text: String,
}

/// Returns `true` when the input contains tag-like HTML markup.
#[must_use]
pub fn looks_like_html(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return false;
    }

    let bytes = trimmed.as_bytes();
    bytes.windows(2).any(|window| {
        window[0] == b'<' && (window[1].is_ascii_alphabetic() || matches!(window[1], b'/' | b'!'))
    }) && trimmed.contains('>')
}

/// Removes HTML comments from the input.
#[must_use]
pub fn strip_html_comments(input: &str) -> String {
    let mut result = String::new();
    let mut remainder = input;

    while let Some(start) = remainder.find("<!--") {
        result.push_str(&remainder[..start]);
        let comment = &remainder[start + 4..];
        if let Some(end) = comment.find("-->") {
            remainder = &comment[end + 3..];
        } else {
            remainder = "";
            break;
        }
    }

    result.push_str(remainder);
    result
}

/// Removes tag-like markup with simple angle-bracket stripping.
#[must_use]
pub fn strip_tags_basic(input: &str) -> String {
    let mut result = String::new();
    let mut inside_tag = false;

    for character in input.chars() {
        match character {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => result.push(character),
            _ => {},
        }
    }

    result
}

/// Escapes common HTML-sensitive characters.
#[must_use]
pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Unescapes common HTML entities.
#[must_use]
pub fn unescape_html(input: &str) -> String {
    input
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

/// Extracts anchor tags with `href` attributes.
#[must_use]
pub fn extract_links(input: &str) -> Vec<HtmlLink> {
    let lower = input.to_ascii_lowercase();
    let mut results = Vec::new();
    let mut search_start = 0;

    while let Some(start_offset) = lower[search_start..].find("<a") {
        let start = search_start + start_offset;
        let Some(open_end_offset) = lower[start..].find('>') else {
            break;
        };
        let open_end = start + open_end_offset;
        let Some(close_offset) = lower[open_end + 1..].find("</a>") else {
            break;
        };
        let close_start = open_end + 1 + close_offset;
        let element = &input[start..=open_end];
        if let Some(href) = get_attribute(element, "href") {
            let text = strip_tags_basic(&input[open_end + 1..close_start])
                .trim()
                .to_string();
            results.push(HtmlLink { text, href });
        }

        search_start = close_start + 4;
    }

    results
}

/// Extracts heading tags in document order.
#[must_use]
pub fn extract_headings(input: &str) -> Vec<HtmlHeading> {
    let lower = input.to_ascii_lowercase();
    let mut results = Vec::new();
    let mut search_start = 0;

    while let Some(start_offset) = lower[search_start..].find("<h") {
        let start = search_start + start_offset;
        let bytes = lower.as_bytes();
        let Some(level_byte) = bytes.get(start + 2) else {
            break;
        };
        if !(b'1'..=b'6').contains(level_byte) {
            search_start = start + 2;
            continue;
        }

        let after_level = bytes.get(start + 3).copied();
        if let Some(after_level) = after_level {
            if after_level != b'>' && !after_level.is_ascii_whitespace() {
                search_start = start + 2;
                continue;
            }
        }

        let Some(open_end_offset) = lower[start..].find('>') else {
            break;
        };
        let open_end = start + open_end_offset;
        let level = level_byte - b'0';
        let close_tag = format!("</h{level}>");
        let Some(close_offset) = lower[open_end + 1..].find(&close_tag) else {
            break;
        };
        let close_start = open_end + 1 + close_offset;
        let text = strip_tags_basic(&input[open_end + 1..close_start])
            .trim()
            .to_string();
        results.push(HtmlHeading { level, text });
        search_start = close_start + close_tag.len();
    }

    results
}

/// Extracts the title text from the first `<title>` tag.
#[must_use]
pub fn extract_title(input: &str) -> Option<String> {
    let lower = input.to_ascii_lowercase();
    let start = lower.find("<title>")? + 7;
    let end = lower[start..].find("</title>")? + start;
    Some(strip_tags_basic(&input[start..end]).trim().to_string())
}

/// Extracts a `<meta name="..." content="...">` value.
#[must_use]
pub fn extract_meta_content(input: &str, name: &str) -> Option<String> {
    let lower = input.to_ascii_lowercase();
    let mut search_start = 0;

    while let Some(start_offset) = lower[search_start..].find("<meta") {
        let start = search_start + start_offset;
        let end_offset = lower[start..].find('>')?;
        let end = start + end_offset;
        let element = &input[start..=end];
        if get_attribute(element, "name")
            .as_deref()
            .is_some_and(|value| value.eq_ignore_ascii_case(name))
        {
            return get_attribute(element, "content");
        }
        search_start = end + 1;
    }

    None
}

/// Extracts attributes from an opening tag.
#[must_use]
pub fn extract_attributes(element: &str) -> Vec<HtmlAttribute> {
    let trimmed = element.trim();
    if !trimmed.starts_with('<') {
        return Vec::new();
    }

    let mut inner = trimmed.trim_start_matches('<').trim_end_matches('>').trim();
    inner = inner.strip_suffix('/').unwrap_or(inner).trim_end();

    let mut index = 0;
    let bytes = inner.as_bytes();
    while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
        index += 1;
    }

    let mut attributes = Vec::new();
    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() {
            break;
        }

        let name_start = index;
        while index < bytes.len() && !bytes[index].is_ascii_whitespace() && bytes[index] != b'=' {
            index += 1;
        }
        let name = inner[name_start..index].trim();
        if name.is_empty() {
            break;
        }

        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }

        let value = if index < bytes.len() && bytes[index] == b'=' {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                index += 1;
            }
            if index >= bytes.len() {
                Some(String::new())
            } else {
                let quote = bytes[index];
                if quote == b'\'' || quote == b'"' {
                    index += 1;
                    let value_start = index;
                    while index < bytes.len() && bytes[index] != quote {
                        index += 1;
                    }
                    let parsed = inner[value_start..index].to_string();
                    if index < bytes.len() {
                        index += 1;
                    }
                    Some(parsed)
                } else {
                    let value_start = index;
                    while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
                        index += 1;
                    }
                    Some(inner[value_start..index].to_string())
                }
            }
        } else {
            None
        };

        attributes.push(HtmlAttribute {
            name: name.to_ascii_lowercase(),
            value,
        });
    }

    attributes
}

/// Returns the named attribute value from an element when present.
#[must_use]
pub fn get_attribute(element: &str, name: &str) -> Option<String> {
    let requested = name.trim().to_ascii_lowercase();
    extract_attributes(element)
        .into_iter()
        .find(|attribute| attribute.name == requested)
        .and_then(|attribute| attribute.value)
}
