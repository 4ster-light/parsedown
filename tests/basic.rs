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
fn test_paragraphs() {
    let parser = Parser::new();
    assert_eq!(
        parser.parse("This is a paragraph."),
        "<p>This is a paragraph.</p>"
    );
    assert_eq!(
        parser.parse("This is\na multi-line\nparagraph."),
        "<p>This is</p>\n<p>a multi-line</p>\n<p>paragraph.</p>"
    );
}
