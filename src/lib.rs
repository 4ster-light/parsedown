use std::fmt::Write;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Text(String),
    Bold(Vec<Token>),
    Italic(Vec<Token>),
    Heading(u8, Vec<Token>),
    ListItem(Vec<Token>),
    List(Vec<Vec<Token>>),
    Paragraph(Vec<Token>),
    Newline,
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&self, input: &str) -> String {
        let tokens = self.tokenize(input);
        self.generate_html(tokens)
    }

    fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut lines = input.lines().peekable();
        let mut current_list: Vec<Vec<Token>> = Vec::new();

        while let Some(line) = lines.next() {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                if !current_list.is_empty() {
                    tokens.push(Token::List(current_list));
                    current_list = Vec::new();
                }
                tokens.push(Token::Newline);
                continue;
            }

            if let Some(level) = self.get_heading_level(trimmed_line) {
                if !current_list.is_empty() {
                    tokens.push(Token::List(current_list));
                    current_list = Vec::new();
                }
                let content = trimmed_line.trim_start_matches('#').trim();
                tokens.push(Token::Heading(level, self.format_inline(content)));
            } else if trimmed_line.starts_with("- ") {
                current_list.push(vec![Token::ListItem(self.format_inline(&trimmed_line[2..].trim()))]);
            } else {
                if !current_list.is_empty() {
                    tokens.push(Token::List(current_list));
                    current_list = Vec::new();
                }
                tokens.push(Token::Paragraph(self.format_inline(trimmed_line)));
            }
        }

        if !current_list.is_empty() {
            tokens.push(Token::List(current_list));
        }

        self.merge_paragraphs(tokens)
    }

    fn get_heading_level(&self, line: &str) -> Option<u8> {
        let level = line.chars().take_while(|&c| c == '#').count();
        if level > 0 && level <= 6 && line.chars().nth(level) == Some(' ') {
            Some(level as u8)
        } else {
            None
        }
    }

    fn format_inline(&self, text: &str) -> Vec<Token> {
        let mut result = Vec::new();
        let mut chars = text.chars().peekable();
        
        while let Some(c) = chars.next() {
            match c {
                '*' => {
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        let (content, rest) = self.consume_until_double_asterisk(&mut chars);
                        result.push(Token::Bold(self.format_inline(&content)));
                        chars = rest;
                    } else {
                        let (content, rest) = self.consume_until_single_asterisk(&mut chars);
                        result.push(Token::Italic(self.format_inline(&content)));
                        chars = rest;
                    }
                }
                _ => {
                    let mut text = String::new();
                    text.push(c);
                    while let Some(&next_c) = chars.peek() {
                        if next_c == '*' {
                            break;
                        }
                        text.push(chars.next().unwrap());
                    }
                    if !text.is_empty() {
                        result.push(Token::Text(text));
                    }
                }
            }
        }

        result
    }

    fn consume_until_double_asterisk<I>(&self, chars: &mut I) -> (String, I)
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut content = String::new();
        let mut rest = chars.clone();

        while let Some(c) = chars.next() {
            if c == '*' && chars.next() == Some('*') {
                return (content, rest);
            }
            content.push(c);
            rest = chars.clone();
        }

        (content, rest)
    }

    fn consume_until_single_asterisk<I>(&self, chars: &mut I) -> (String, I)
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut content = String::new();
        let mut rest = chars.clone();

        while let Some(c) = chars.next() {
            if c == '*' {
                return (content, rest);
            }
            content.push(c);
            rest = chars.clone();
        }

        (content, rest)
    }

    fn merge_paragraphs(&self, tokens: Vec<Token>) -> Vec<Token> {
        let mut merged = Vec::new();
        let mut current_paragraph = Vec::new();

        for token in tokens {
            match token {
                Token::Paragraph(content) => {
                    if !current_paragraph.is_empty() {
                        current_paragraph.push(Token::Text(" ".to_string()));
                    }
                    current_paragraph.extend(content);
                }
                Token::Newline => {
                    if !current_paragraph.is_empty() {
                        merged.push(Token::Paragraph(current_paragraph));
                        current_paragraph = Vec::new();
                    }
                    merged.push(Token::Newline);
                }
                _ => {
                    if !current_paragraph.is_empty() {
                        merged.push(Token::Paragraph(current_paragraph));
                        current_paragraph = Vec::new();
                    }
                    merged.push(token);
                }
            }
        }

        if !current_paragraph.is_empty() {
            merged.push(Token::Paragraph(current_paragraph));
        }

        merged
    }

    fn generate_html(&self, tokens: Vec<Token>) -> String {
        let mut output = String::new();

        for token in tokens {
            match token {
                Token::Paragraph(content) => {
                    write!(output, "<p>").unwrap();
                    self.write_inline_content(&mut output, &content);
                    writeln!(output, "</p>").unwrap();
                }
                Token::Heading(level, content) => {
                    write!(output, "<h{}>", level).unwrap();
                    self.write_inline_content(&mut output, &content);
                    writeln!(output, "</h{}>", level).unwrap();
                }
                Token::List(items) => {
                    writeln!(output, "<ul>").unwrap();
                    for item in items {
                        if let Some(Token::ListItem(content)) = item.get(0) {
                            write!(output, "<li>").unwrap();
                            self.write_inline_content(&mut output, content);
                            writeln!(output, "</li>").unwrap();
                        }
                    }
                    writeln!(output, "</ul>").unwrap();
                }
                Token::Newline => {}
                _ => {}
            }
        }

        output.trim().to_string()
    }

    fn write_inline_content(&self, output: &mut String, content: &[Token]) {
        for token in content {
            match token {
                Token::Text(text) => write!(output, "{}", text).unwrap(),
                Token::Bold(tokens) => {
                    write!(output, "<strong>").unwrap();
                    self.write_inline_content(output, tokens);
                    write!(output, "</strong>").unwrap();
                }
                Token::Italic(tokens) => {
                    write!(output, "<em>").unwrap();
                    self.write_inline_content(output, tokens);
                    write!(output, "</em>").unwrap();
                }
                _ => {}
            }
        }
    }
}
