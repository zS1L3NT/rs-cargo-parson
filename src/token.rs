use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::OpenCurlyBracket => write!(f, "{{"),
            TokenType::CloseCurlyBracket => write!(f, "}}"),
            TokenType::OpenSquareBracket => write!(f, "["),
            TokenType::CloseSquareBracket => write!(f, "]"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma => write!(f, ","),
            TokenType::String(string) => write!(f, "\"{}\"", string),
            TokenType::Number(number) => write!(f, "{}", number),
            TokenType::Boolean(boolean) => write!(f, "{}", boolean),
            TokenType::Null => write!(f, "null"),
        }
    }
}
