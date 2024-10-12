pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&self, input: &str) -> String {
        // Simple implementation that converts markdown header to HTML
        input
            .lines()
            .map(|line| {
                if line.starts_with("# ") {
                    let content = &line[2..];
                    format!("<h1>{}</h1>", content)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_heading() {
        let parser = Parser::new();
        let markdown = "# Hello world!";
        let expected_html = "<h1>Hello world!</h1>";
        assert_eq!(parser.parse(markdown), expected_html);
    }

    #[test]
    fn test_no_heading() {
        let parser = Parser::new();
        let markdown = "Hello world!";
        let expected_html = "Hello world!";
        assert_eq!(parser.parse(markdown), expected_html);
    }
}
