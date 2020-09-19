use super::token_type::TokenType;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, line: usize) -> Token {
        Token { kind, lexeme, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal = match &self.kind {
            TokenType::String(value) => value.clone(),
            TokenType::Number(value) => value.to_string(),
            _ => "".into(),
        };

        let kind = match &self.kind {
            TokenType::String(_) => "String".into(),
            TokenType::Number(_) => "Number".into(),
            t => format!("{:?}", t),
        };
        write!(f, "{} {} {}", kind, self.lexeme, literal)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_display_for_common_token() {
        let token = Token::new(TokenType::Comma, ",".into(), 10);

        assert_eq!(token.to_string(), "Comma , ");
    }

    #[test]
    fn correct_display_for_literal_token() {
        let token = Token::new(TokenType::String("Example text".into()), "\"".into(), 10);

        assert_eq!(token.to_string(), "String \" Example text");
    }
}
