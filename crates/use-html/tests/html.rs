use use_html::{
    escape_html, extract_attributes, extract_headings, extract_links, extract_meta_content,
    extract_title, get_attribute, looks_like_html, strip_html_comments, strip_tags_basic,
    unescape_html,
};

#[test]
fn detects_html() {
    assert!(looks_like_html("<p>Hello</p>"));
    assert!(!looks_like_html("plain text"));
}

#[test]
fn strips_comments_and_tags() {
    assert_eq!(
        strip_html_comments("<p>Hello</p><!-- note --><p>World</p>"),
        "<p>Hello</p><p>World</p>"
    );
    assert_eq!(
        strip_tags_basic("<p>Hello <strong>world</strong></p>"),
        "Hello world"
    );
}

#[test]
fn escapes_and_unescapes_html() {
    assert_eq!(
        escape_html("<a href=\"/\">Tom & Jerry</a>"),
        "&lt;a href=&quot;/&quot;&gt;Tom &amp; Jerry&lt;/a&gt;"
    );
    assert_eq!(
        unescape_html("&lt;strong&gt;Hi&lt;/strong&gt; &amp; bye"),
        "<strong>Hi</strong> & bye"
    );
}

#[test]
fn extracts_links_and_headings() {
    let html = "<h1>Docs</h1><p><a href=\"/guide\">Guide</a></p><h2>Install</h2>";

    let links = extract_links(html);
    let headings = extract_headings(html);

    assert_eq!(links.len(), 1);
    assert_eq!(links[0].href, "/guide");
    assert_eq!(links[0].text, "Guide");
    assert_eq!(headings.len(), 2);
    assert_eq!(headings[0].level, 1);
    assert_eq!(headings[1].text, "Install");
}

#[test]
fn extracts_title_and_meta_content() {
    let html = "<html><head><title>Docs</title><meta name=\"description\" content=\"Reference\"></head></html>";

    assert_eq!(extract_title(html).as_deref(), Some("Docs"));
    assert_eq!(
        extract_meta_content(html, "description").as_deref(),
        Some("Reference")
    );
}

#[test]
fn extracts_attributes() {
    let attrs = extract_attributes("<a href=\"/guide\" target=\"_blank\" data-id=\"42\" disabled>");

    assert_eq!(attrs.len(), 4);
    assert_eq!(
        get_attribute("<a href=\"/guide\" target=\"_blank\">", "href").as_deref(),
        Some("/guide")
    );
    assert_eq!(
        get_attribute("<a data-id=\"42\">", "data-id").as_deref(),
        Some("42")
    );
}

#[test]
fn handles_malformed_and_empty_input() {
    assert!(extract_links("<a href=\"/guide\">Guide").is_empty());
    assert!(extract_headings("").is_empty());
    assert_eq!(extract_title(""), None);
}
