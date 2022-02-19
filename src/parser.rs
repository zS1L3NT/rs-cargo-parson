use crate::{
    token::Token, JSONArray, JSONBoolean, JSONNull, JSONNumber, JSONObject, JSONString, JSONValue,
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&self) -> JSONValue {
        let mut tokens = self.tokens.clone();

        if let Some(first_token) = tokens.first() {
            match first_token {
                Token::OpenSquareBracket => {
                    let result = self
                        .parse_array(&mut tokens)
                        .expect("Invalid JSON: First token of array not [");
                    JSONValue::from_array(result)
                }
                Token::OpenCurlyBracket => {
                    let result = self
                        .parse_object(&mut tokens)
                        .expect("Invalid JSON: First token of object not {");
                    JSONValue::from_object(result)
                }
                Token::String(string) => {
                    if tokens.len() == 1 {
                        JSONValue::from_string(JSONString::new(string.clone()))
                    } else {
                        panic!("Invalid JSON: Expected end of file after \"{}\"", string)
                    }
                }
                Token::Number(number) => {
                    if tokens.len() == 1 {
                        JSONValue::from_number(JSONNumber::new(*number))
                    } else {
                        panic!("Invalid JSON: Expected end of file after {}", number)
                    }
                }
                Token::Boolean(boolean) => {
                    if tokens.len() == 1 {
                        JSONValue::from_boolean(JSONBoolean::new(*boolean))
                    } else {
                        panic!("Invalid JSON: Expected end of file after {}", boolean)
                    }
                }
                Token::Null => {
                    if tokens.len() == 1 {
                        JSONValue::from_null(JSONNull::new())
                    } else {
                        panic!("Invalid JSON: Expected end of file after null")
                    }
                }
                Token::CloseCurlyBracket => {
                    panic!("Invalid JSON: Unexpected start of file: {{")
                }
                Token::CloseSquareBracket => {
                    panic!("Invalid JSON: Unexpected start of file: }}")
                }
                Token::Colon => {
                    panic!("Invalid JSON: Unexpected start of file: :")
                }
                Token::Comma => {
                    panic!("Invalid JSON: Unexpected start of file: ,")
                }
            }
        } else {
            panic!("Invalid JSON: No tokens found");
        }
    }

    fn parse_value(&self, tokens: &mut Vec<Token>, stop_tokens: &Vec<Token>) -> Option<JSONValue> {
        if let Some(results) = self.parse_string(tokens, &stop_tokens) {
            return Some(JSONValue::from_string(results.0));
        }

        if let Some(results) = self.parse_number(tokens, &stop_tokens) {
            return Some(JSONValue::from_number(results.0));
        }

        if let Some(results) = self.parse_boolean(tokens, &stop_tokens) {
            return Some(JSONValue::from_boolean(results.0));
        }

        if let Some(results) = self.parse_null(tokens, &stop_tokens) {
            return Some(JSONValue::from_null(results.0));
        }

        if let Some(json_array) = self.parse_array(tokens) {
            return Some(JSONValue::from_array(json_array));
        }

        if let Some(json_object) = self.parse_object(tokens) {
            return Some(JSONValue::from_object(json_object));
        }

        return None;
    }

    fn parse_string(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<(JSONString, Token)> {
        let tokens_cl = tokens.clone();

        match tokens_cl.first() {
            Some(Token::String(string)) => {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_tokens.contains(second_token) {
                        tokens.remove(0);
                        return Some((JSONString::new(string.clone()), second_token.clone()));
                    }
                }
                panic!(
                    "Invalid JSON: Unexpected end of file after string: {}",
                    string
                )
            }
            _ => None,
        }
    }

    fn parse_number(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<(JSONNumber, Token)> {
        let tokens_cl = tokens.clone();

        match tokens_cl.first() {
            Some(Token::Number(number)) => {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_tokens.contains(second_token) {
                        tokens.remove(0);
                        return Some((JSONNumber::new(*number), second_token.clone()));
                    }
                }
                panic!(
                    "Invalid JSON: Unexpected end of file after number: {}",
                    number
                )
            }
            _ => None,
        }
    }

    fn parse_boolean(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<(JSONBoolean, Token)> {
        let tokens_cl = tokens.clone();

        match tokens_cl.first() {
            Some(Token::Boolean(boolean)) => {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_tokens.contains(second_token) {
                        tokens.remove(0);
                        return Some((JSONBoolean::new(*boolean), second_token.clone()));
                    }
                }
                panic!(
                    "Invalid JSON: Unexpected end of file after boolean: {}",
                    boolean
                )
            }
            _ => None,
        }
    }

    fn parse_null(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<(JSONNull, Token)> {
        let tokens_cl = tokens.clone();

        match tokens_cl.first() {
            Some(Token::Null) => {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_tokens.contains(second_token) {
                        tokens.remove(0);
                        return Some((JSONNull::new(), second_token.clone()));
                    }
                }
                panic!("Invalid JSON: Unexpected end of file after null")
            }
            _ => None,
        }
    }

    fn parse_array(&self, tokens: &mut Vec<Token>) -> Option<JSONArray> {
        match tokens.first() {
            Some(Token::OpenSquareBracket) => {
                tokens.remove(0);
                let mut array = JSONArray::new();

                match tokens.first() {
                    Some(Token::CloseSquareBracket) => {
                        tokens.remove(0);
                        return Some(array);
                    }
                    _ => {}
                }

                while tokens.len() > 0 {
                    array.push(
                        self.parse_value(tokens, &vec![Token::CloseSquareBracket, Token::Comma])
                            .expect("Invalid JSON: Could not parse value"),
                    );

                    match tokens.first() {
                        Some(Token::CloseSquareBracket) => {
                            tokens.remove(0);
                            return Some(array);
                        }
                        Some(Token::Comma) => {
                            tokens.remove(0);
                        }
                        Some(char) => {
                            panic!("Invalid JSON: Unexpected character: {}", char)
                        }
                        _ => {
                            panic!("Invalid JSON: Unexpected end of file")
                        }
                    }
                }

                panic!("Invalid JSON: Unexpected end of file before finding open square bracket");
            }
            _ => None,
        }
    }

    fn parse_object(&self, tokens: &mut Vec<Token>) -> Option<JSONObject> {
        match tokens.first() {
            Some(Token::OpenCurlyBracket) => {
                tokens.remove(0);
                let mut object = JSONObject::new();

                match tokens.first() {
                    Some(Token::CloseCurlyBracket) => {
                        tokens.remove(0);
                        return Some(object);
                    }
                    _ => {}
                }

                while tokens.len() > 0 {
                    let tokens_cl = tokens.clone();
                    let key = tokens_cl.first().unwrap();
                    let key = match key {
                        Token::String(string) => {
                            tokens.remove(0);
                            string.clone()
                        }
                        _ => {
                            panic!("Invalid JSON: Expected string key in object, got {}", key);
                        }
                    };

                    match tokens.first() {
                        Some(Token::Colon) => {
                            tokens.remove(0);
                        }
                        Some(token) => {
                            panic!(
                                "Invalid JSON: Expected colon after key in object, got {}",
                                token
                            );
                        }
                        None => {
                            panic!("Invalid JSON: Unexpected end of file")
                        }
                    }

                    object.set(
                        key,
                        self.parse_value(tokens, &vec![Token::CloseCurlyBracket, Token::Comma])
                            .expect("Invalid JSON: Could not parse value"),
                    );

                    match tokens.first() {
                        Some(Token::CloseCurlyBracket) => {
                            tokens.remove(0);
                            return Some(object);
                        }
                        Some(Token::Comma) => {
                            tokens.remove(0);
                        }
                        Some(char) => {
                            panic!("Invalid JSON: Unexpected character: {}", char)
                        }
                        _ => {
                            panic!("Invalid JSON: Unexpected end of file")
                        }
                    }
                }

                panic!("Invalid JSON: Unexpected end of file before finding open curly bracket");
            }
            _ => None,
        }
    }
}
