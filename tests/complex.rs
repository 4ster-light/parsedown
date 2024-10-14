use parsedown::Parser;

#[test]
fn test_complex_markdown() {
    let parser = Parser::new();
    let markdown = r#"
# Heading 1

This is a paragraph with **bold** and *italic* text.

## Heading 2

- List item 1
- List item 2 with *emphasis*
- **Bold** list item 3

### Heading 3

Another paragraph here.
"#;
    let expected_html = r#"<h1>Heading 1</h1>
<p>This is a paragraph with <strong>bold</strong> and <em>italic</em> text.</p>
<h2>Heading 2</h2>
<ul>
<li>List item 1</li>
<li>List item 2 with <em>emphasis</em></li>
<li><strong>Bold</strong> list item 3</li>
</ul>
<h3>Heading 3</h3>
<p>Another paragraph here.</p>"#;

    assert_eq!(parser.parse(markdown), expected_html);
}
