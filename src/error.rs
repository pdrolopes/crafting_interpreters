use crate::token::Token;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum LoxError {
    ParserError(usize, String),
    RuntimeError(Token, String),
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::ParserError(line, reason) => {
                write!(f, "Parser error in line {}: {}", line, reason)
            }
            LoxError::RuntimeError(token, message) => {
                write!(f, "Runtime error: {} \n [line {}]", message, token.line)
            }
        }
    }
}

impl std::error::Error for LoxError {}

pub type Result<T> = std::result::Result<T, LoxError>;
