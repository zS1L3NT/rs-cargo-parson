use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::OpenCurlyBracket => write!(f, "{{"),
            Token::CloseCurlyBracket => write!(f, "}}"),
            Token::OpenSquareBracket => write!(f, "["),
            Token::CloseSquareBracket => write!(f, "]"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::String(string) => write!(f, "\"{}\"", string),
            Token::Number(number) => write!(f, "{}", number),
            Token::Boolean(boolean) => write!(f, "{}", boolean),
            Token::Null => write!(f, "null"),
        }
    }
}
