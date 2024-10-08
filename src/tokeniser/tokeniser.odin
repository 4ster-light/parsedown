package tokeniser

import "core:strings"

Token_Type :: enum {
    Heading,
    Paragraph,
    Bold,
    Italic,
    Link,
    LineBreak,
}

Token :: struct {
    type:    Token_Type,
    content: string,
    level:   int, // For headings
    url:     string, // For links
}

tokenise :: proc(input: string) -> [dynamic]Token {
    tokens := make([dynamic]Token)
    i := 0
    in_paragraph := false

    for i < len(input) {
        switch input[i] {
        case '#':
            i = process_heading(input, &i, &tokens, &in_paragraph)
        case '*', '_':
            i = process_emphasis(input, &i, &tokens)
        case '[':
            i = process_link(input, &i, &tokens, &in_paragraph)
        case '\n':
            i = process_newline(input, &i, &tokens, &in_paragraph)
        case:
            i = process_text(input, &i, &tokens, &in_paragraph)
        }
    }

    end_paragraph(&tokens)
    return tokens
}

process_heading :: proc(input: string, i: ^int, tokens: ^[dynamic]Token, in_paragraph: ^bool) -> int {
    end_paragraph(tokens)
    level := 1
    for i^ + level < len(input) && input[i^ + level] == '#' {
        level += 1
    }
    start := i^ + level
    end := start
    for end < len(input) && input[end] != '\n' {
        end += 1
    }
    append(tokens, Token{type = .Heading, content = strings.trim_space(input[start:end]), level = level})
    in_paragraph^ = false
    return end + 1
}

process_emphasis :: proc(input: string, i: ^int, tokens: ^[dynamic]Token) -> int {
    if i^ + 1 < len(input) && input[i^ + 1] == input[i^] {
        // Bold
        start := i^ + 2
        end := start
        for end < len(input) && (input[end] != input[i^] || input[end + 1] != input[i^]) {
            end += 1
        }
        append(tokens, Token{type = .Bold, content = input[start:end]})
        return end + 2
    } else {
        // Italic
        start := i^ + 1
        end := start
        for end < len(input) && input[end] != input[i^] {
            end += 1
        }
        append(tokens, Token{type = .Italic, content = input[start:end]})
        return end + 1
    }
}

process_link :: proc(input: string, i: ^int, tokens: ^[dynamic]Token, in_paragraph: ^bool) -> int {
    link_text_start := i^ + 1
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
        append(tokens, Token{
            type = .Link,
            content = input[link_text_start:link_text_end],
            url = input[url_start:url_end],
        })
        return url_end + 1
    } else {
        if !in_paragraph^ {
            append(tokens, Token{type = .Paragraph, content = ""})
            in_paragraph^ = true
        }
        append(tokens, Token{type = .Paragraph, content = input[i^:link_text_end + 1]})
        return link_text_end + 1
    }
}

process_newline :: proc(input: string, i: ^int, tokens: ^[dynamic]Token, in_paragraph: ^bool) -> int {
    if i^ + 1 < len(input) && input[i^ + 1] == '\n' {
        end_paragraph(tokens)
        in_paragraph^ = false
        return i^ + 2
    } else {
        if in_paragraph^ {
            last_token := &tokens[len(tokens)-1]
            last_token.content = strings.concatenate({last_token.content, " "})
        }
        return i^ + 1
    }
}

process_text :: proc(input: string, i: ^int, tokens: ^[dynamic]Token, in_paragraph: ^bool) -> int {
    if !in_paragraph^ {
        append(tokens, Token{type = .Paragraph, content = ""})
        in_paragraph^ = true
    }
    start := i
    for i^ < len(input) && input[i^] != '#' && input[i^] != '*' && input[i^] != '_' && input[i^] != '[' && input[i^] != '\n' {
        i^ += 1
    }
    if i > start {
        last_token := &tokens[len(tokens)-1]
        last_token.content = strings.concatenate({last_token.content, input[start^:i^]})
    }
    return i^
}

end_paragraph :: proc(tokens: ^[dynamic]Token) {
    if len(tokens) > 0 && tokens[len(tokens)-1].type == .Paragraph {
        last_token := &tokens[len(tokens)-1]
        last_token.content = strings.trim_space(last_token.content)
    }
}
