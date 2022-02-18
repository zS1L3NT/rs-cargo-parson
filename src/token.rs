use std::fmt::Display;

#[derive(Debug)]
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

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Self::OpenCurlyBracket => Self::OpenCurlyBracket,
            Self::CloseCurlyBracket => Self::CloseCurlyBracket,
            Self::OpenSquareBracket => Self::OpenSquareBracket,
            Self::CloseSquareBracket => Self::CloseSquareBracket,
            Self::Colon => Self::Colon,
            Self::Comma => Self::Comma,
            Self::String(string) => Self::String(string.clone()),
            Self::Number(number) => Self::Number(number.clone()),
            Self::Boolean(boolean) => Self::Boolean(boolean.clone()),
            Self::Null => Self::Null,
        }
    }
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

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::Boolean(left), Self::Boolean(right)) => left == right,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
