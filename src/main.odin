package main

import "core:fmt"
import "core:os"
import "core:strings"
import "core:slice"

import "tokeniser"
import "parser"

main :: proc() {
    markdown := `# Hello World

This is a paragraph with **bold** and *italic* text.

Check out this [link](https://example.com)!

## Another heading
This is a paragraph right after a heading.`

    tokens := tokeniser.tokenise(markdown)
    defer delete(tokens)

    parsed := parser.parse(tokens)
    fmt.println(parsed)

    // Uncomment the following to read from a file instead
    /*
    markdown, ok := os.read_entire_file("./test.md")
    if !ok {
        fmt.println("Could not read file")
        return
    }
    defer delete(markdown)

    tokens := tokenizer.tokenise(string(markdown))
    defer delete(tokens)

    parsed := parser.parse(tokens)
    fmt.println(parsed)
    */
}
