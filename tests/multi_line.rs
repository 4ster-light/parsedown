use parsedown::Parser;

#[test]
fn test_lists() {
    let parser = Parser::new();
    let input = "- Item 1\n- Item 2\n- Item 3";
    let expected = "<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n<li>Item 3</li>\n</ul>";
    assert_eq!(parser.parse(input), expected);
}

#[test]
fn test_paragraphs() {
    let parser = Parser::new();
    assert_eq!(
        parser.parse("This is a paragraph."),
        "<p>This is a paragraph.</p>"
    );
    assert_eq!(
        parser.parse("This is\n\na multi-line\n\nparagraph."),
        "<p>This is</p>\n<p>a multi-line</p>\n<p>paragraph.</p>"
    );
}
