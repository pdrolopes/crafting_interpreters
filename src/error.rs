use std::fmt::Display;
#[derive(Debug, PartialEq)]
pub enum LoxError {
    ParserError(usize, String),
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::ParserError(line, reason) => {
                write!(f, "Parser error in line {}: {}", line, reason)
            }
        }
    }
}

impl std::error::Error for LoxError {}

pub type Result<T> = std::result::Result<T, LoxError>;
