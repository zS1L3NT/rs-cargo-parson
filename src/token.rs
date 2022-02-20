use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl PartialEq for Location {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Location {
    pub fn new() -> Self {
        Location { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenCurlyBracket(Location),
    CloseCurlyBracket(Location),
    OpenSquareBracket(Location),
    CloseSquareBracket(Location),
    Colon(Location),
    Comma(Location),
    String(String, Location),
    Number(f64, Location),
    Boolean(bool, Location),
    Null(Location),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::OpenCurlyBracket(location) => write!(f, "{{"),
            Token::CloseCurlyBracket(location) => write!(f, "}}"),
            Token::OpenSquareBracket(location) => write!(f, "["),
            Token::CloseSquareBracket(location) => write!(f, "]"),
            Token::Colon(location) => write!(f, ":"),
            Token::Comma(location) => write!(f, ","),
            Token::String(string, location) => write!(f, "\"{}\"", string),
            Token::Number(number, location) => write!(f, "{}", number),
            Token::Boolean(boolean, location) => write!(f, "{}", boolean),
            Token::Null(location) => write!(f, "null"),
        }
    }
}
