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
            let result = self.lex_string(json.clone());
            let string_token = result.0;
            json = result.1;
            if string_token.is_some() {
                tokens.push(string_token.unwrap());
                continue;
            }

            let result = self.lex_number(json.clone());
            let number_token = result.0;
            json = result.1;
            if number_token.is_some() {
                tokens.push(number_token.unwrap());
                continue;
            }

            let result = self.lex_boolean(json.clone());
            let boolean_token = result.0;
            json = result.1;
            if boolean_token.is_some() {
                tokens.push(boolean_token.unwrap());
                continue;
            }

            let result = self.lex_null(json.clone());
            let null_token = result.0;
            json = result.1;
            if null_token.is_some() {
                tokens.push(null_token.unwrap());
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

    fn lex_string(&self, string: String) -> (Option<Token>, String) {
        let mut data = String::new();

        if &string[..1] != "\"" {
            return (None, string);
        }

        let mut escape = false;

        for char in string[1..].chars() {
            if escape {
                if ['r', 'n', 't', '\\', '\"'].contains(&char) {
                    data.push(char);
					escape = false;
                } else {
                    panic!("Invalid JSON: Invalid escape of character: <{}>", char.encode_utf8(&mut [0, 0]));
                }
            } else {
                if char == '\\' {
					data.push(char);
                    escape = true;
                } else if char == '"' {
                    let len = data.len() + 2;
                    return (Some(Token::String(data)), format!("{}", &string[len..]));
                } else {
                    data.push(char);
                }
            }
        }

        (None, string)
    }

    fn lex_number(&self, string: String) -> (Option<Token>, String) {
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
            return (None, string);
        }

        let len = data.len();
        return (
            Some(Token::Number(data.parse().unwrap())),
            format!("{}", &string[len..]),
        );
    }

    fn lex_boolean(&self, string: String) -> (Option<Token>, String) {
        if string.len() > 4 && string.starts_with("true") {
            (Some(Token::Boolean(true)), format!("{}", &string[4..]))
        } else if string.len() > 5 && string.starts_with("false") {
            (Some(Token::Boolean(false)), format!("{}", &string[5..]))
        } else {
            (None, string)
        }
    }

    fn lex_null(&self, string: String) -> (Option<Token>, String) {
        if string.len() > 4 && string.starts_with("null") {
            (Some(Token::Null), format!("{}", &string[4..]))
        } else {
            (None, string)
        }
    }
}
