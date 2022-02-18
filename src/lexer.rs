use crate::token::Token;

pub struct Lexer {
    pub json: String,
}

impl Lexer {
    pub fn new(json: String) -> Lexer {
        Lexer { json }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut json = self.json.clone();

        while json.len() > 0 {
            if let Some(string_token) = self.lex_string(&mut json) {
                tokens.push(string_token);
                continue;
            }

            if let Some(number_token) = self.lex_number(&mut json) {
                tokens.push(number_token);
                continue;
            }

            if let Some(boolean_token) = self.lex_boolean(&mut json) {
                tokens.push(boolean_token);
                continue;
            }

            if let Some(null_token) = self.lex_null(&mut json) {
                tokens.push(null_token);
                continue;
            }

            let first_char = json[..1].to_string();
            match &first_char[..] {
                " " | "\t" | "\r" | "\n" => {
                    json = format!("{}", &json[1..]);
                }
                "{" => {
                    tokens.push(Token::OpenCurlyBracket);
                    json = format!("{}", &json[1..]);
                }
                "}" => {
                    tokens.push(Token::CloseCurlyBracket);
                    json = format!("{}", &json[1..]);
                }
                "[" => {
                    tokens.push(Token::OpenSquareBracket);
                    json = format!("{}", &json[1..]);
                }
                "]" => {
                    tokens.push(Token::CloseSquareBracket);
                    json = format!("{}", &json[1..]);
                }
                "," => {
                    tokens.push(Token::Comma);
                    json = format!("{}", &json[1..]);
                }
                ":" => {
                    tokens.push(Token::Colon);
                    json = format!("{}", &json[1..]);
                }
                "\"" => {
                    panic!("Unexpected end of file: {}", json);
                }
                char => {
                    panic!("Unexpected character: {}", char);
                }
            }
        }

        tokens
    }

    fn lex_string(&self, string: &mut String) -> Option<Token> {
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
                    panic!(
                        "Invalid JSON: Invalid escape of character: <{}>",
                        char.encode_utf8(&mut [0, 0])
                    );
                }
            } else {
                if char == '\\' {
                    data.push(char);
                    escape = true;
                } else if char == '"' {
                    *string = format!("{}", &string[data.len() + 2..]);
                    return Some(Token::String(data));
                } else {
                    data.push(char);
                }
            }
        }

        None
    }

    fn lex_number(&self, string: &mut String) -> Option<Token> {
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
                panic!("Invalid JSON: Invalid position for character \"-\" in number");
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
                panic!("Invalid JSON: Invalid position for character \"e\" in number")
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
                panic!("Invalid JSON: Invalid position for character \".\" in number");
            }

            break;
        }

        if data.len() == 0 {
            return None;
        }

        *string = format!("{}", &string[data.len()..]);
        Some(Token::Number(data.parse().unwrap()))
    }

    fn lex_boolean(&self, string: &mut String) -> Option<Token> {
        if string.len() > 4 && string.starts_with("true") {
            *string = format!("{}", &string[4..]);
            Some(Token::Boolean(true))
        } else if string.len() > 5 && string.starts_with("false") {
            *string = format!("{}", &string[5..]);
            Some(Token::Boolean(false))
        } else {
            None
        }
    }

    fn lex_null(&self, string: &mut String) -> Option<Token> {
        if string.len() > 4 && string.starts_with("null") {
            *string = format!("{}", &string[4..]);
            Some(Token::Null)
        } else {
            None
        }
    }
}
