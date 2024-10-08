package parser

import "core:strings"
import "core:fmt"

import "../tokeniser"

parse :: proc(tokens: [dynamic]tokeniser.Token) -> string {
    builder := strings.builder_make()

    for token in tokens {
        switch token.type {
        case .Heading:
            strings.write_string(&builder, fmt.tprintf("<h%d>%s</h%d>", token.level, token.content, token.level))
        case .Paragraph:
            strings.write_string(&builder, fmt.tprintf("<p>%s</p>", token.content))
        case .Bold:
            strings.write_string(&builder, fmt.tprintf("<strong>%s</strong>", token.content))
        case .Italic:
            strings.write_string(&builder, fmt.tprintf("<em>%s</em>", token.content))
        case .Link:
            strings.write_string(&builder, fmt.tprintf("<a href=\"%s\">%s</a>", token.url, token.content))
        case .LineBreak:
            strings.write_string(&builder, "<br>")
        }
    }

    return strings.to_string(builder)
}
