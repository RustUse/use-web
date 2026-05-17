#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A lightweight CSS declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssDeclaration {
    pub property: String,
    pub value: String,
}

/// A lightweight CSS rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssRule {
    pub selector: String,
    pub declarations: Vec<CssDeclaration>,
}

/// Returns `true` when the input looks like CSS markup or a declaration.
#[must_use]
pub fn looks_like_css(input: &str) -> bool {
    let trimmed = input.trim();
    !trimmed.is_empty()
        && ((trimmed.contains('{') && trimmed.contains('}'))
            || split_css_declaration(trimmed).is_some())
}

/// Returns `true` when the input looks like a CSS identifier.
#[must_use]
pub fn is_css_identifier(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.starts_with("--") {
        return false;
    }

    let mut characters = trimmed.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || matches!(first, '_' | '-')) {
        return false;
    }

    characters.all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

/// Returns `true` when the input looks like a CSS custom property.
#[must_use]
pub fn is_css_custom_property(input: &str) -> bool {
    let trimmed = input.trim();
    trimmed.starts_with("--")
        && trimmed.len() > 2
        && trimmed[2..]
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

/// Splits a `property: value` declaration.
#[must_use]
pub fn split_css_declaration(input: &str) -> Option<CssDeclaration> {
    let trimmed = input.trim().trim_end_matches(';').trim();
    let (property, value) = trimmed.split_once(':')?;
    let property = normalize_css_property(property);
    let value = value.trim();
    if value.is_empty() || !(is_css_identifier(&property) || is_css_custom_property(&property)) {
        return None;
    }

    Some(CssDeclaration {
        property,
        value: value.to_string(),
    })
}

/// Extracts declarations from a CSS block or declaration list.
#[must_use]
pub fn extract_css_declarations(input: &str) -> Vec<CssDeclaration> {
    let without_comments = strip_css_comments(input);
    let declaration_source = if let Some(start) = without_comments.find('{') {
        let end = without_comments
            .rfind('}')
            .unwrap_or(without_comments.len());
        &without_comments[start + 1..end]
    } else {
        without_comments.as_str()
    };

    declaration_source
        .split(';')
        .filter_map(split_css_declaration)
        .collect()
}

/// Normalizes a CSS property to lowercase ASCII.
#[must_use]
pub fn normalize_css_property(input: &str) -> String {
    input.trim().to_ascii_lowercase()
}

/// Returns `true` when the input looks like a common CSS color value.
#[must_use]
pub fn is_css_color_value(input: &str) -> bool {
    let trimmed = input.trim().to_ascii_lowercase();
    if let Some(hex) = trimmed.strip_prefix('#') {
        return matches!(hex.len(), 3 | 4 | 6 | 8)
            && hex.chars().all(|character| character.is_ascii_hexdigit());
    }

    trimmed.starts_with("rgb(")
        || trimmed.starts_with("rgba(")
        || trimmed.starts_with("hsl(")
        || trimmed.starts_with("hsla(")
        || matches!(
            trimmed.as_str(),
            "black"
                | "white"
                | "red"
                | "green"
                | "blue"
                | "gray"
                | "grey"
                | "transparent"
                | "currentcolor"
        )
}

/// Returns `true` when the input looks like a common CSS length value.
#[must_use]
pub fn is_css_length_value(input: &str) -> bool {
    let trimmed = input.trim().to_ascii_lowercase();
    if trimmed == "0" || trimmed == "0.0" {
        return true;
    }

    let split_at = trimmed
        .char_indices()
        .find(|(_, character)| {
            !(character.is_ascii_digit() || matches!(character, '.' | '+' | '-'))
        })
        .map_or(trimmed.len(), |(index, _)| index);
    let (number, unit) = trimmed.split_at(split_at);
    !number.is_empty()
        && number.parse::<f64>().is_ok()
        && matches!(
            unit,
            "px" | "rem"
                | "em"
                | "vw"
                | "vh"
                | "vmin"
                | "vmax"
                | "ch"
                | "ex"
                | "cm"
                | "mm"
                | "in"
                | "pt"
                | "pc"
                | "%"
        )
}

/// Removes CSS comments from the input.
#[must_use]
pub fn strip_css_comments(input: &str) -> String {
    let mut result = String::new();
    let mut remainder = input;

    while let Some(start) = remainder.find("/*") {
        result.push_str(&remainder[..start]);
        let comment = &remainder[start + 2..];
        if let Some(end) = comment.find("*/") {
            remainder = &comment[end + 2..];
        } else {
            remainder = "";
            break;
        }
    }

    result.push_str(remainder);
    result
}

/// Performs a small whitespace-oriented CSS minification pass.
#[must_use]
pub fn minify_css_basic(input: &str) -> String {
    let without_comments = strip_css_comments(input);
    let mut collapsed = String::new();
    let mut previous_was_space = false;

    for character in without_comments.chars() {
        if character.is_whitespace() {
            if !previous_was_space {
                collapsed.push(' ');
                previous_was_space = true;
            }
        } else {
            collapsed.push(character);
            previous_was_space = false;
        }
    }

    let mut compact = collapsed.trim().to_string();
    for (from, to) in [
        (" {", "{"),
        ("{ ", "{"),
        (" }", "}"),
        ("; ", ";"),
        (": ", ":"),
        (" :", ":"),
        (", ", ","),
        (" >", ">"),
        ("> ", ">"),
        (" +", "+"),
        ("+ ", "+"),
        (" ~", "~"),
        ("~ ", "~"),
    ] {
        compact = compact.replace(from, to);
    }

    compact
}
