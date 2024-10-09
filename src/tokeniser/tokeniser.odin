package tokenizer

import "core:strings"
import "core:mem"

Token :: struct {
    kind: TokenKind,
    text: string,
    url: string,
}

TokenKind :: enum {
    Text,
    Header,
    Bold,
    Italic,
    Link,
}

tokenise :: proc(input: string, allocator := context.allocator) -> []Token {
    tokens := make([dynamic]Token, allocator)
    remaining := input

    for len(remaining) > 0 {
        if token := parse_header(&remaining); token != nil {
            append(&tokens, token^)
        } else if token := parse_bold(&remaining); token != nil {
            append(&tokens, token^)
        } else if token := parse_italic(&remaining); token != nil {
            append(&tokens, token^)
        } else if token := parse_link(&remaining); token != nil {
            append(&tokens, token^)
        } else {
            append(&tokens, parse_text(&remaining))
        }
    }

    return tokens[:]
}

parse_header :: proc(input: ^string, allocator := context.allocator) -> ^Token {
    if strings.has_prefix(input^, "#") {
        end := strings.index_byte(input^, '\n')
        if end == -1 {
            end = len(input^)
        }
        text := input^[:end]
        input^ = input^[end:]
        token := new(Token, allocator)
        token^ = Token{kind = .Header, text = text}
        return token
    }
    return nil
}

parse_bold :: proc(input: ^string, allocator := context.allocator) -> ^Token {
    if strings.has_prefix(input^, "**") {
        end := strings.index(input^[2:], "**")
        if end != -1 {
            text := input^[2:end+2]
            input^ = input^[end+4:]
            token := new(Token, allocator)
            token^ = Token{kind = .Bold, text = text}
            return token
        }
    }
    return nil
}

parse_italic :: proc(input: ^string, allocator := context.allocator) -> ^Token {
    if strings.has_prefix(input^, "*") {
        end := strings.index_byte(input^[1:], '*')
        if end != -1 {
            text := input^[1:end+1]
            input^ = input^[end+2:]
            token := new(Token, allocator)
            token^ = Token{kind = .Italic, text = text}
            return token
        }
    }
    return nil
}

parse_link :: proc(input: ^string, allocator := context.allocator) -> ^Token {
    if strings.has_prefix(input^, "[") {
        text_end := strings.index_byte(input^, ']')
        if text_end != -1 && strings.has_prefix(input^[text_end+1:], "(") {
            url_end := strings.index_byte(input^[text_end+2:], ')')
            if url_end != -1 {
                text := input^[1:text_end]
                url := input^[text_end+2 : text_end+2+url_end]
                input^ = input^[text_end+3+url_end:]
                token := new(Token, allocator)
                token^ = Token{kind = .Link, text = text, url = url}
                return token
            }
        }
    }
    return nil
}

parse_text :: proc(input: ^string) -> Token {
    end := strings.index_any(input^, "#*[")
    if end == -1 {
        end = len(input^)
    }
    text := input^[:end]
    input^ = input^[end:]
    return Token{kind = .Text, text = text}
}
