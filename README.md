# Parsedown
A markdown parser for written in Rust for HTML generation.

## Usage
First, add this to your `Cargo.toml`:
```toml
[dependencies]
parsedown = "0.1.0"
```

Then, add this to your crate root:
```rust
extern crate parsedown;
```

Finally, use it like this:
```rust
let input = "# Hello World";
let parser = parsedown::Parser::new();
let output = parser.parse(input);
```

## Features
(For now it is a work in progress, it supports basic Markdown syntax)

- Text
- Bold
- Italic
- Headings
- Lists
- Paragraphs

## License
MIT
