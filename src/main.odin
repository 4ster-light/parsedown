package main

import "core:fmt"
import "core:os"
import "core:strings"
import "core:slice"

Token_Type :: enum {
    Heading,
    Text,
    Bold,
    Italic,
    Link,
}

Token :: struct {
    type:    Token_Type,
    content: string,
    level:   int,       // For headings
    url:     string,    // For links
}

tokenise :: proc(input: string) -> [dynamic]Token {
    tokens := make([dynamic]Token)
    
    i := 0
    for i < len(input) {
        switch input[i] {
        case '#':
            level := 1
            for i + level < len(input) && input[i + level] == '#' {
                level += 1
            }
            start := i + level
            end := start
            for end < len(input) && input[end] != '\n' {
                end += 1
            }
            append(&tokens, Token{type = .Heading, content = strings.trim_space(input[start:end]), level = level})
            i = end + 1

        case '*', '_':
            if i + 1 < len(input) && input[i + 1] == input[i] {
                // Bold
                start := i + 2
                end := start
                for end < len(input) && (input[end] != input[i] || input[end + 1] != input[i]) {
                    end += 1
                }
                append(&tokens, Token{type = .Bold, content = input[start:end]})
                i = end + 2
            } else {
                // Italic
                start := i + 1
                end := start
                for end < len(input) && input[end] != input[i] {
                    end += 1
                }
                append(&tokens, Token{type = .Italic, content = input[start:end]})
                i = end + 1
            }

        case '[':
            link_text_start := i + 1
            link_text_end := link_text_start
            for link_text_end < len(input) && input[link_text_end] != ']' {
                link_text_end += 1
            }
            if link_text_end + 2 < len(input) && input[link_text_end + 1] == '(' {
                url_start := link_text_end + 2
                url_end := url_start
                for url_end < len(input) && input[url_end] != ')' {
                    url_end += 1
                }
                append(&tokens, Token{
                    type = .Link,
                    content = input[link_text_start:link_text_end],
                    url = input[url_start:url_end],
                })
                i = url_end + 1
            } else {
                append(&tokens, Token{type = .Text, content = input[i:link_text_end + 1]})
                i = link_text_end + 1
            }

        case:
            start := i
            for i < len(input) && input[i] != '#' && input[i] != '*' && input[i] != '_' && input[i] != '[' {
                i += 1
            }
            if i > start {
                append(&tokens, Token{type = .Text, content = input[start:i]})
            }
        }
    }

    return tokens
}

parse :: proc(tokens: [dynamic]Token) -> string {
    builder := strings.builder_make()

    for token in tokens {
        switch token.type {
        case .Heading:
            strings.write_string(&builder, fmt.tprintf("<h%d>%s</h%d>", token.level, token.content, token.level))
        case .Text:
            strings.write_string(&builder, token.content)
        case .Bold:
            strings.write_string(&builder, fmt.tprintf("<strong>%s</strong>", token.content))
        case .Italic:
            strings.write_string(&builder, fmt.tprintf("<em>%s</em>", token.content))
        case .Link:
            strings.write_string(&builder, fmt.tprintf("<a href=\"%s\">%s</a>", token.url, token.content))
        }
    }

    return strings.to_string(builder)
}

main :: proc() {
    markdown := `# Hello World

This is a paragraph with **bold** and *italic* text.

Check out this [link](https://example.com)!

## Another heading`

    tokens := tokenise(markdown)
    defer delete(tokens)

    parsed := parse(tokens)
    fmt.println(parsed)
}
