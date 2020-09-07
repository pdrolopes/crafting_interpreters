use super::lox;
use super::token::Token;
use super::token_type::TokenType;

pub struct Scanner {
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    source: String,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    #[allow(dead_code)]
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".into(), self.line));
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => {
                let token = if self.a_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            }

            '=' => {
                let token = if self.a_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            }

            '<' => {
                let token = if self.a_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            }

            '>' => {
                let token = if self.a_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            }
            '/' => {
                if self.a_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' => {} // do nothing for theses chars
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            x => lox::error(self.line, &format!("Unexpected character. '{}'", x)),
        };
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        // unterminated string
        if self.is_at_end() {
            lox::error(self.line, "Unterminated string.");
            return;
        }

        // the closing "
        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token(TokenType::String(value.into()));
    }

    fn a_match(&mut self, expected: char) -> bool {
        // match is a rust keyword
        if self.is_at_end() {
            return false;
        };
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        };

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap() //current will never pass the size of source
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap() //current will never pass the size of source
    }

    fn add_token(&mut self, kind: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(kind, text.to_string(), self.line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_scan() {
        let source = r#"// this is a comment
                        (( )){} // grouping stuff
                        !*+-/=<> <= == // operators"#;

        let mut scanner = Scanner::new(source.into());
        scanner.scan_tokens();

        let token_types: Vec<TokenType> = scanner
            .tokens
            .iter()
            .map(|token| token.kind.clone())
            .collect();

        let expected = vec![
            TokenType::LeftParen,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Bang,
            TokenType::Star,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Equal,
            TokenType::Less,
            TokenType::Greater,
            TokenType::LessEqual,
            TokenType::EqualEqual,
            TokenType::Eof,
        ];
        assert_eq!(token_types, expected);
    }

    #[test]
    fn string_scan() {
        let source = r#"
                "a little string"
            "#;

        let mut scanner = Scanner::new(source.into());
        scanner.scan_tokens();

        let token_types: Vec<TokenType> = scanner
            .tokens
            .iter()
            .map(|token| token.kind.clone())
            .collect();

        assert_eq!(
            token_types,
            vec![TokenType::String("a little string".into()), TokenType::Eof]
        )
    }
}
