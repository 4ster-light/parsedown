# Parsedown
A Markdown parser to `HTML` written in [Odin](https://github.com/odin-lang/Odin).

## Using the program

### Requirements
- Odin
- Make (optional)

### Building
```bash
make
```

### Running
```bash
make run
```

### If you don't have make
```bash
odin build src/main.odin -out:build/parsedown -file
./build/parsedown
```

> [!NOTE]
> The markdown should be edited in the `src/main.odin` file


- Code (main procedure):
```odin
main :: proc() {
    markdown := `# Hello World

    This is a paragraph with **bold** and *italic* text.

    Check out this [link](https://example.com)!

    ## Another heading`

    tokens := parsedown.tokenise(markdown)
    defer delete(tokens)

    parsed := parsedown.parse(tokens)
    fmt.println(parsed)
}
```

- Output:

```html
<h1>Hello World</h1>
<p>This is a paragraph with <strong>bold</strong> and <em>italic</em> text.</p>
<p>Check out this <a href="https://example.com">link</a>!</p>
<h2>Another heading</h2>
```