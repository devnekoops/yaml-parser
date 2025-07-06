use std::fmt;

#[derive(Debug)]
pub enum YamlError {
    ParseError(String),
    IndentationError(String),
    InvalidValue(String),
    UnexpectedChar { char: char, line: usize, column: usize },
    UnexpectedEof,
}

impl fmt::Display for YamlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YamlError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            YamlError::IndentationError(msg) => write!(f, "Indentation error: {}", msg),
            YamlError::InvalidValue(msg) => write!(f, "Invalid Value error: {}", msg),
            YamlError::UnexpectedChar { char, line, column } => {
                write!(f, "Unexpected character '{}' at line {}, column {}", char, line, column)
            }
            YamlError::UnexpectedEof => write!(f, "Unexpected end of file"),
        }
    }
}

impl std::error::Error for YamlError {}

pub type Result<T> = std::result::Result<T, YamlError>;