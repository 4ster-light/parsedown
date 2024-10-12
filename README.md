# Parsedown
A markdown parser for written in Rust for HTML generation.

## Usage
```rust
extern crate parsedown;

use parsedown::Parser;

let mut parser = Parser::new();
let markdown = "# Hello world!";
let html = parser.parse(markdown);
```

## Features
(For now it is a work in progress)
