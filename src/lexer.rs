use crate::{json_err, token::Token, JSONError};

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

        while json.len() > 0 {
            if let Some(result) = self.lex_string(&mut json) {
                match result {
                    Ok(string_token) => {
                        tokens.push(string_token);
                        continue;
                    }
                    Err(json_error) => return Err(json_error),
                }
            }

            if let Some(result) = self.lex_number(&mut json) {
                match result {
                    Ok(number_token) => {
                        tokens.push(number_token);
                        continue;
                    }
                    Err(json_error) => return Err(json_error),
                }
            }

            if let Some(result) = self.lex_boolean(&mut json) {
                match result {
                    Ok(boolean_token) => {
                        tokens.push(boolean_token);
                        continue;
                    }
                    Err(json_error) => return Err(json_error),
                }
            }

            if let Some(result) = self.lex_null(&mut json) {
                match result {
                    Ok(null_token) => {
                        tokens.push(null_token);
                        continue;
                    }
                    Err(json_error) => return Err(json_error),
                }
            }

            let first_char = json[..1].to_string();
            match &first_char[..] {
                " " | "\t" | "\r" | "\n" => {
                    json = json[1..].to_string();
                }
                "{" => {
                    tokens.push(Token::OpenCurlyBracket);
                    json = json[1..].to_string();
                }
                "}" => {
                    tokens.push(Token::CloseCurlyBracket);
                    json = json[1..].to_string();
                }
                "[" => {
                    tokens.push(Token::OpenSquareBracket);
                    json = json[1..].to_string();
                }
                "]" => {
                    tokens.push(Token::CloseSquareBracket);
                    json = json[1..].to_string();
                }
                "," => {
                    tokens.push(Token::Comma);
                    json = json[1..].to_string();
                }
                ":" => {
                    tokens.push(Token::Colon);
                    json = json[1..].to_string();
                }
                "\"" => {
                    return json_err!("Unexpected end of file: {}", json);
                }
                char => {
                    return json_err!("Unexpected character: {}", char);
                }
            }
        }

        Ok(tokens)
    }

    fn lex_string(&self, string: &mut String) -> Option<Result<Token, JSONError>> {
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
                    return Some(json_err!(
                        "Invalid JSON: Invalid escape of character: <{}>",
                        char.encode_utf8(&mut [0, 0])
                    ));
                }
            } else {
                if char == '\\' {
                    data.push(char);
                    escape = true;
                } else if char == '"' {
                    *string = string[data.len() + 2..].to_string();
                    return Some(Ok(Token::String(data)));
                } else {
                    data.push(char);
                }
            }
        }

        None
    }

    fn lex_number(&self, string: &mut String) -> Option<Result<Token, JSONError>> {
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
                return Some(json_err!(
                    "Invalid JSON: Invalid position for character \"-\" in number"
                ));
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
                return Some(json_err!(
                    "Invalid JSON: Invalid position for character \"e\" in number"
                ));
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
                return Some(json_err!(
                    "Invalid JSON: Invalid position for character \".\" in number"
                ));
            }

            break;
        }

        if data.len() == 0 {
            return None;
        }

        *string = string[data.len()..].to_string();
        Some(Ok(Token::Number(data.parse().unwrap())))
    }

    fn lex_boolean(&self, string: &mut String) -> Option<Result<Token, JSONError>> {
        if string.len() > 4 && string.starts_with("true") {
            *string = string[4..].to_string();
            Some(Ok(Token::Boolean(true)))
        } else if string.len() > 5 && string.starts_with("false") {
            *string = string[5..].to_string();
            Some(Ok(Token::Boolean(false)))
        } else {
            None
        }
    }

    fn lex_null(&self, string: &mut String) -> Option<Result<Token, JSONError>> {
        if string.len() > 4 && string.starts_with("null") {
            *string = string[4..].to_string();
            Some(Ok(Token::Null))
        } else {
            None
        }
    }
}
