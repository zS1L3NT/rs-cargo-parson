use crate::{
    json_err,
    token::{Token, TokenType},
    JSONError,
};

pub struct Lexer {
    pub json: String,
}

impl Lexer {
    pub fn new(json: String) -> Self {
        Lexer { json }
    }

    pub fn get_tokens(&self) -> Result<Vec<Token>, JSONError> {
        let mut tokens = vec![];
        let mut json = self.json.clone();
        let mut line = 1;
        let mut column = 1;

        while json.len() > 0 {
            if let Some(result) = self.lex_string(&mut json, line, column) {
                if let Ok(result) = result {
                    let (string_token, length) = result;
                    tokens.push(string_token);
                    column += length;
                    continue;
                } else {
                    json_err!(result.unwrap_err());
                }
            }

            if let Some(result) = self.lex_number(&mut json, line, column) {
                if let Ok(result) = result {
                    let (number_token, length) = result;
                    tokens.push(number_token);
                    column += length;
                    continue;
                } else {
                    json_err!(result.unwrap_err());
                }
            }

            if let Some(result) = self.lex_boolean(&mut json, line, column) {
                if let Ok(result) = result {
                    let (boolean_token, length) = result;
                    tokens.push(boolean_token);
                    column += length;
                    continue;
                } else {
                    json_err!(result.unwrap_err());
                }
            }

            if let Some(result) = self.lex_null(&mut json, line, column) {
                if let Ok(result) = result {
                    let (null_token, length) = result;
                    tokens.push(null_token);
                    column += length;
                    continue;
                } else {
                    json_err!(result.unwrap_err());
                }
            }

            let first_char = json[..1].to_string();
            match &first_char[..] {
                " " | "\t" => {
                    json = json[1..].to_string();
                    column += 1;
                }
                "\r" => {
                    json = json[1..].to_string();
                }
                "\n" => {
                    json = json[1..].to_string();
                    line += 1;
                    column = 1;
                }
                "{" => {
                    json = json[1..].to_string();
                    tokens.push(Token::new(TokenType::OpenCurlyBracket, line, column));
                    column += 1;
                }
                "}" => {
                    json = json[1..].to_string();
                    tokens.push(Token::new(TokenType::CloseCurlyBracket, line, column));
                    column += 1;
                }
                "[" => {
                    json = json[1..].to_string();
                    tokens.push(Token::new(TokenType::OpenSquareBracket, line, column));
                    column += 1;
                }
                "]" => {
                    json = json[1..].to_string();
                    tokens.push(Token::new(TokenType::CloseSquareBracket, line, column));
                    column += 1;
                }
                "," => {
                    json = json[1..].to_string();
                    tokens.push(Token::new(TokenType::Comma, line, column));
                    column += 1;
                }
                ":" => {
                    json = json[1..].to_string();
                    tokens.push(Token::new(TokenType::Colon, line, column));
                    column += 1;
                }
                "\"" => {
                    json_err!("Unexpected end of file <{}>", json; line, column);
                }
                char => {
                    json_err!("Unexpected character <{}>", char; line, column);
                }
            }
        }

        Ok(tokens)
    }

    fn lex_string(
        &self,
        string: &mut String,
        line: usize,
        column: usize,
    ) -> Option<Result<(Token, usize), JSONError>> {
        let mut data = String::new();

        if &string[..1] != "\"" {
            return None;
        }

        let mut escape = false;

        for char in string[1..].chars() {
            if escape {
                if ['r', 'n', 't', '\\', '\"'].contains(&char) {
                    data.push(char);
                    escape = false;
                } else {
                    json_err!(
                        Some; "Invalid escape of character <{}>", char.encode_utf8(&mut [0, 0]); line, column
                    )
                }
            } else {
                if char == '\\' {
                    data.push(char);
                    escape = true;
                } else if char == '"' {
                    let length = data.len();
                    *string = string[length + 2..].to_string();
                    return Some(Ok((
                        Token::new(TokenType::String(data), line, column),
                        length + 2,
                    )));
                } else {
                    data.push(char);
                }
            }
        }

        json_err!(Some; "Unexpected end of string"; line, column)
    }

    fn lex_number(
        &self,
        string: &mut String,
        line: usize,
        column: usize,
    ) -> Option<Result<(Token, usize), JSONError>> {
        let mut data = String::new();

        for char in string.chars() {
            let last_char = data.chars().last();
            if char.is_digit(10) {
                data.push(char);
                continue;
            }

            if char == '-' {
                if data == "" || last_char.unwrap() == 'e' || last_char.unwrap() == 'E' {
                    data.push(char);
                    continue;
                }
                json_err!(
                    Some;
                    "Invalid character in number <->";
                    line,
                    column
                );
            }

            if char == 'e' || char == 'E' {
                if data != ""
                    && last_char.unwrap().is_digit(10)
                    && !data.contains('e')
                    && !data.contains('E')
                {
                    data.push(char);
                    continue;
                }
                json_err!(
                    Some;
                    "Invalid character in number <e>";
                    line,
                    column
                );
            }

            if char == '.' {
                if data != ""
                    && last_char.unwrap().is_digit(10)
                    && !data.contains("e")
                    && !data.contains('E')
                {
                    data.push(char);
                    continue;
                }
                json_err!(
                    Some;
                    "Invalid character in number <.>";
                    line,
                    column
                );
            }

            break;
        }

        if data.len() == 0 {
            return None;
        }

        *string = string[data.len()..].to_string();
        Some(Ok((
            Token::new(TokenType::Number(data.parse().unwrap()), line, column),
            data.len(),
        )))
    }

    fn lex_boolean(
        &self,
        string: &mut String,
        line: usize,
        column: usize,
    ) -> Option<Result<(Token, usize), JSONError>> {
        if string.len() > 4 && string.starts_with("true") {
            *string = string[4..].to_string();
            Some(Ok((Token::new(TokenType::Boolean(true), line, column), 4)))
        } else if string.len() > 5 && string.starts_with("false") {
            *string = string[5..].to_string();
            Some(Ok((Token::new(TokenType::Boolean(false), line, column), 5)))
        } else {
            None
        }
    }

    fn lex_null(
        &self,
        string: &mut String,
        line: usize,
        column: usize,
    ) -> Option<Result<(Token, usize), JSONError>> {
        if string.len() > 4 && string.starts_with("null") {
            *string = string[4..].to_string();
            Some(Ok((Token::new(TokenType::Null, line, column), 4)))
        } else {
            None
        }
    }
}
