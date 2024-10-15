use std::fmt::Write;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Text(String),
    Bold(TokenList),
    Italic(TokenList),
    Heading { level: u8, content: TokenList },
    List(Vec<TokenList>),
    Paragraph(TokenList),
}

#[derive(Debug, PartialEq, Clone, Default)]
struct TokenList(Vec<Token>);

impl TokenList {
    fn new() -> Self {
        TokenList(Vec::new())
    }

    fn push(&mut self, token: Token) {
        self.0.push(token);
    }

    fn extend(&mut self, other: TokenList) {
        self.0.extend(other.0);
    }
}

struct Tokenizer;

pub struct Parser;

impl Tokenizer {
    fn tokenize(&self, input: &str) -> TokenList {
        let mut tokens = TokenList::new();
        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if let Some(level) = self.get_heading_level(trimmed) {
                let content = trimmed.trim_start_matches('#').trim();
                tokens.push(Token::Heading {
                    level,
                    content: self.tokenize_inline(content),
                });
            } else if trimmed.starts_with("- ") {
                let mut list_items = Vec::new();
                list_items.push(self.tokenize_inline(&trimmed[2..]));

                while let Some(next_line) = lines.peek() {
                    if next_line.trim().starts_with("- ") {
                        list_items.push(self.tokenize_inline(&lines.next().unwrap()[2..]));
                    } else {
                        break;
                    }
                }
                tokens.push(Token::List(list_items));
            } else {
                let mut paragraph = self.tokenize_inline(trimmed);
                while let Some(next_line) = lines.peek() {
                    let next_trimmed = next_line.trim();
                    if !next_trimmed.is_empty() && !next_trimmed.starts_with('#') && !next_trimmed.starts_with("- ") {
                        paragraph.push(Token::Text(" ".to_string()));
                        paragraph.extend(self.tokenize_inline(next_trimmed));
                        lines.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Paragraph(paragraph));
            }
        }

        tokens
    }

    fn tokenize_inline(&self, text: &str) -> TokenList {
        let mut tokens = TokenList::new();
        let mut chars = text.chars().peekable();
        let mut current_text = String::new();

        while let Some(c) = chars.next() {
            match c {
                '*' => {
                    if !current_text.is_empty() {
                        tokens.push(Token::Text(current_text));
                        current_text = String::new();
                    }
                    if chars.peek() == Some(&'*') {
                        chars.next(); // Skip the second '*'
                        let (content, _) = self.consume_until_delimiter(&mut chars, "**");
                        tokens.push(Token::Bold(self.tokenize_inline(&content)));
                    } else {
                        let (content, _) = self.consume_until_delimiter(&mut chars, "*");
                        tokens.push(Token::Italic(self.tokenize_inline(&content)));
                    }
                }
                _ => {
                    current_text.push(c);
                }
            }
        }

        if !current_text.is_empty() {
            tokens.push(Token::Text(current_text));
        }

        tokens
    }

    fn get_heading_level(&self, line: &str) -> Option<u8> {
        let level = line.chars().take_while(|&c| c == '#').count();
        if level > 0 && level <= 6 && line.chars().nth(level) == Some(' ') {
            Some(level as u8)
        } else {
            None
        }
    }

    fn consume_until_delimiter<I>(&self, chars: &mut I, delimiter: &str) -> (String, I)
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut content = String::new();
        let mut rest = chars.clone();
        let delimiter_chars: Vec<_> = delimiter.chars().collect();
        let mut matched = 0;

        while let Some(c) = chars.next() {
            if c == delimiter_chars[matched] {
                matched += 1;
                if matched == delimiter_chars.len() {
                    return (content, rest);
                }
            } else {
                content.push_str(&delimiter_chars[..matched].iter().collect::<String>());
                content.push(c);
                matched = 0;
            }
            rest = chars.clone();
        }

        (content, rest)
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&self, input: &str) -> String {
        let tokenizer = Tokenizer;
        let tokens = tokenizer.tokenize(input);
        self.generate_html(&tokens)
    }

    fn generate_html(&self, tokens: &TokenList) -> String {
        let mut output = String::new();

        for token in &tokens.0 {
            match token {
                Token::Paragraph(content) => {
                    writeln!(output, "<p>{}</p>", self.generate_html(content)).unwrap();
                }
                Token::Heading { level, content } => {
                    writeln!(output, "<h{0}>{1}</h{0}>", level, self.generate_html(content)).unwrap();
                }
                Token::List(items) => {
                    writeln!(output, "<ul>").unwrap();
                    for item in items {
                        writeln!(output, "<li>{}</li>", self.generate_html(item)).unwrap();
                    }
                    writeln!(output, "</ul>").unwrap();
                }
                Token::Bold(content) => {
                    write!(output, "<strong>{}</strong>", self.generate_html(content)).unwrap();
                }
                Token::Italic(content) => {
                    write!(output, "<em>{}</em>", self.generate_html(content)).unwrap();
                }
                Token::Text(text) => {
                    output.push_str(text);
                }
            }
        }

        output.trim().to_string()
    }
}
