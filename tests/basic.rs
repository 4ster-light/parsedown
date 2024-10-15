use parsedown::Parser;

#[test]
fn test_headings() {
    let parser = Parser::new();
    assert_eq!(parser.parse("# Heading 1"), "<h1>Heading 1</h1>");
    assert_eq!(parser.parse("## Heading 2"), "<h2>Heading 2</h2>");
    assert_eq!(parser.parse("### Heading 3"), "<h3>Heading 3</h3>");
    assert_eq!(parser.parse("###### Heading 6"), "<h6>Heading 6</h6>");
}

#[test]
fn test_inline_formatting() {
    let parser = Parser::new();
    assert_eq!(
        parser.parse("This is **bold** and *italic* text."),
        "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>"
    );
}
