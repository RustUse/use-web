use use_css::{
    extract_css_declarations, is_css_color_value, is_css_custom_property, is_css_identifier,
    is_css_length_value, looks_like_css, minify_css_basic, normalize_css_property,
    split_css_declaration, strip_css_comments,
};

#[test]
fn detects_css() {
    assert!(looks_like_css(".card { color: red; }"));
    assert!(looks_like_css("color: red"));
    assert!(!looks_like_css("plain text"));
}

#[test]
fn splits_and_extracts_declarations() {
    let declaration = split_css_declaration("color: red;").unwrap();
    let declarations = extract_css_declarations(".card { color: red; margin-top: 1rem; }");

    assert_eq!(declaration.property, "color");
    assert_eq!(declarations.len(), 2);
}

#[test]
fn normalizes_properties_and_detects_custom_properties() {
    assert_eq!(normalize_css_property(" Margin-Top "), "margin-top");
    assert!(is_css_identifier("margin-top"));
    assert!(is_css_custom_property("--brand-color"));
}

#[test]
fn detects_color_and_length_values() {
    assert!(is_css_color_value("#ff00aa"));
    assert!(is_css_color_value("rgba(10, 20, 30, 0.5)"));
    assert!(is_css_length_value("1.5rem"));
    assert!(is_css_length_value("0"));
}

#[test]
fn strips_comments_and_minifies() {
    assert_eq!(
        strip_css_comments(".card { color: red; /* note */ margin: 1rem; }"),
        ".card { color: red;  margin: 1rem; }"
    );
    assert_eq!(
        minify_css_basic(".card { color: red; margin-top: 1rem; }"),
        ".card{color:red;margin-top:1rem;}"
    );
}

#[test]
fn handles_malformed_and_empty_input() {
    assert_eq!(split_css_declaration("color red"), None);
    assert!(extract_css_declarations("").is_empty());
    assert!(!is_css_identifier("1bad"));
    assert!(!is_css_length_value("big"));
}
