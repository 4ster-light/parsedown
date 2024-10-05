package main

import "core:fmt"
import "core:os"

Token_Type :: enum {
    Heading,
    Text,
    Bold,
    Link
}

Token :: struct {
    type: Token_Type,
    content: string,
}

tokenise :: proc(input: string) -> []Token {
    return []Token{}
}

parse :: proc(tokens: []Token) -> string {
    return "";
}


main :: proc() {
    markdown := ""

    tokens := tokenise(markdown)
    parsed := parse(tokens)

    fmt.println(parsed)
}
