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
    pub fn new(source: String) -> Scanner {
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
            '?' => self.add_token(TokenType::Question),
            ':' => self.add_token(TokenType::Colon),

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
                    // Line comentaries
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.a_match('*') {
                    // block comentaries
                    while (self.peek() != '*' || self.peek_next() != Some('/')) && !self.is_at_end()
                    {
                        self.advance();
                    }

                    // file ended without closing block comment
                    if !(self.a_match('*') && self.a_match('/')) {
                        lox::error(self.line, "Unterminated block comment.");
                        return;
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
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
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

        let value: String = self.source[self.start + 1..self.current - 1].into();

        self.add_token(TokenType::String(value));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let is_peek_next_digit = self
            .peek_next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false);
        if self.peek() == '.' && is_peek_next_digit {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Unwrap here is safe because digits are verified in if statements
        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number(value))
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let identifier = &self.source[self.start..self.current];
        let kind = match identifier {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(kind);
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

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
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
    fn string_literals() {
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

    #[test]
    fn number_literals() {
        let source = r#"42 3.7"#;

        let mut scanner = Scanner::new(source.into());
        scanner.scan_tokens();

        let token_types: Vec<TokenType> = scanner
            .tokens
            .iter()
            .map(|token| token.kind.clone())
            .collect();

        assert_eq!(
            token_types,
            vec![
                TokenType::Number(42.0),
                TokenType::Number(3.7),
                TokenType::Eof
            ]
        )
    }

    #[test]
    fn identifier_literals() {
        let source = r#"foo
            _bar
            THIS
            anand
            this
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
            vec![
                TokenType::Identifier,
                TokenType::Identifier,
                TokenType::Identifier,
                TokenType::Identifier,
                TokenType::This,
                TokenType::Eof
            ]
        )
    }

    #[test]
    fn block_commentaries() {
        let source = r#"/* multi
            line
            comentary */
            /****/
            "#;

        let mut scanner = Scanner::new(source.into());
        scanner.scan_tokens();

        let token_types: Vec<TokenType> = scanner
            .tokens
            .iter()
            .map(|token| token.kind.clone())
            .collect();

        assert_eq!(token_types, vec![TokenType::Eof])
    }

    #[test]
    fn block_comments_unfinished() {
        let source = r#"/* comment without finish"#;

        let mut scanner = Scanner::new(source.into());
        scanner.scan_tokens();

        let token_types: Vec<TokenType> = scanner
            .tokens
            .iter()
            .map(|token| token.kind.clone())
            .collect();

        assert_eq!(token_types, vec![TokenType::Eof])
    }
}
