package parser

import "core:strings"
import "core:fmt"

import "../tokeniser"

parse :: proc(tokens: []tokeniser.Token) -> string {
    builder := strings.builder_make()
    defer strings.builder_destroy(&builder)

    for token in tokens {
        switch token.kind {
        case .Text:
            strings.write_string(&builder, token.text)
        case .Header:
            level := count_header_level(token.text)
            strings.write_string(&builder, parse_header(token.text, level))
        case .Bold:
            strings.write_string(&builder, parse_bold(token.text))
        case .Italic:
            strings.write_string(&builder, parse_italic(token.text))
        case .Link:
            strings.write_string(&builder, parse_link(token.text, token.url))
        }
    }

    return strings.to_string(builder)
}

count_header_level :: proc(text: string) -> int {
    level := 0
    for c in text {
        if c == '#' {
            level += 1
        } else {
            break
        }
    }
    return level
}

parse_header :: proc(text: string, level: int) -> string {
    content := strings.trim_space(text[level:])
    return fmt.tprintf("<h%d>%s</h%d>", level, content, level)
}

parse_bold :: proc(text: string) -> string {
    return fmt.tprintf("<strong>%s</strong>", text)
}

parse_italic :: proc(text: string) -> string {
    return fmt.tprintf("<em>%s</em>", text)
}

parse_link :: proc(text: string, url: string) -> string {
    return fmt.tprintf("<a href=\"%s\">%s</a>", url, text)
}
