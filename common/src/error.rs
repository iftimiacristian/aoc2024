use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String),
    NumberParse(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::NumberParse(err)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {msg}"),
            ParseError::NumberParse(err) => write!(f, "Number parse error: {err}"),
        }
    }
}

impl std::error::Error for ParseError {}
