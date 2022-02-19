use crate::{
    json_err, token::Token, JSONArray, JSONBoolean, JSONNull, JSONNumber, JSONObject, JSONResult,
    JSONString, JSONValue,
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&self) -> JSONResult<JSONValue> {
        let mut tokens = self.tokens.clone();

        if let Some(first_token) = tokens.first() {
            match first_token {
                Token::OpenSquareBracket => {
                    let json_array = self.parse_array(&mut tokens);
                    if let Some(json_array) = json_array {
                        if let Ok(json_array) = json_array {
                            Ok(JSONValue::from_array(json_array))
                        } else {
                            json_err!(&json_array.unwrap_err().get_message())
                        }
                    } else {
                        json_err!("Invalid JSON: First token of array not [")
                    }
                }
                Token::OpenCurlyBracket => {
                    let json_object = self.parse_object(&mut tokens);
                    if let Some(json_object) = json_object {
                        if let Ok(json_object) = json_object {
                            Ok(JSONValue::from_object(json_object))
                        } else {
                            json_err!(&json_object.unwrap_err().get_message())
                        }
                    } else {
                        json_err!("Invalid JSON: First token of object not {{")
                    }
                }
                Token::String(string) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_string(JSONString::new(string.clone())))
                    } else {
                        json_err!("Invalid JSON: Expected end of file after \"{}\"", string)
                    }
                }
                Token::Number(number) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_number(JSONNumber::new(*number)))
                    } else {
                        json_err!("Invalid JSON: Expected end of file after {}", number)
                    }
                }
                Token::Boolean(boolean) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_boolean(JSONBoolean::new(*boolean)))
                    } else {
                        json_err!("Invalid JSON: Expected end of file after {}", boolean)
                    }
                }
                Token::Null => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_null(JSONNull::new()))
                    } else {
                        json_err!("Invalid JSON: Expected end of file after null")
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

    fn parse_value(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<JSONResult<JSONValue>> {
        if let Some(result) = self.parse_string(tokens, &stop_tokens) {
            Some(match result {
                Ok(results) => Ok(JSONValue::from_string(results.0)),
                Err(json_error) => Err(json_error),
            })
        } else if let Some(result) = self.parse_number(tokens, &stop_tokens) {
            Some(match result {
                Ok(results) => Ok(JSONValue::from_number(results.0)),
                Err(json_error) => Err(json_error),
            })
        } else if let Some(result) = self.parse_boolean(tokens, &stop_tokens) {
            Some(match result {
                Ok(results) => Ok(JSONValue::from_boolean(results.0)),
                Err(json_error) => Err(json_error),
            })
        } else if let Some(result) = self.parse_null(tokens, &stop_tokens) {
            Some(match result {
                Ok(results) => Ok(JSONValue::from_null(results.0)),
                Err(json_error) => Err(json_error),
            })
        } else if let Some(result) = self.parse_array(tokens) {
            Some(match result {
                Ok(json_array) => Ok(JSONValue::from_array(json_array)),
                Err(json_error) => Err(json_error),
            })
        } else if let Some(result) = self.parse_object(tokens) {
            Some(match result {
                Ok(json_object) => Ok(JSONValue::from_object(json_object)),
                Err(json_error) => Err(json_error),
            })
        } else {
            None
        }
    }

    fn parse_string(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<JSONResult<(JSONString, Token)>> {
        let tokens_cl = tokens.clone();

        match tokens_cl.first() {
            Some(Token::String(string)) => {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_tokens.contains(second_token) {
                        tokens.remove(0);
                        return Some(Ok((JSONString::new(string.clone()), second_token.clone())));
                    }
                }
                Some(json_err!(
                    "Invalid JSON: Unexpected end of file after string: {}",
                    string,
                ))
            }
            _ => None,
        }
    }

    fn parse_number(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<JSONResult<(JSONNumber, Token)>> {
        let tokens_cl = tokens.clone();

        match tokens_cl.first() {
            Some(Token::Number(number)) => {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_tokens.contains(second_token) {
                        tokens.remove(0);
                        return Some(Ok((JSONNumber::new(*number), second_token.clone())));
                    }
                }
                Some(json_err!(
                    "Invalid JSON: Unexpected end of file after number: {}",
                    number,
                ))
            }
            _ => None,
        }
    }

    fn parse_boolean(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<JSONResult<(JSONBoolean, Token)>> {
        let tokens_cl = tokens.clone();

        match tokens_cl.first() {
            Some(Token::Boolean(boolean)) => {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_tokens.contains(second_token) {
                        tokens.remove(0);
                        return Some(Ok((JSONBoolean::new(*boolean), second_token.clone())));
                    }
                }
                Some(json_err!(
                    "Invalid JSON: Unexpected end of file after boolean: {}",
                    boolean,
                ))
            }
            _ => None,
        }
    }

    fn parse_null(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<JSONResult<(JSONNull, Token)>> {
        let tokens_cl = tokens.clone();

        match tokens_cl.first() {
            Some(Token::Null) => {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_tokens.contains(second_token) {
                        tokens.remove(0);
                        return Some(Ok((JSONNull::new(), second_token.clone())));
                    }
                }
                Some(json_err!("Invalid JSON: Unexpected end of file after null"))
            }
            _ => None,
        }
    }

    fn parse_array(&self, tokens: &mut Vec<Token>) -> Option<JSONResult<JSONArray>> {
        match tokens.first() {
            Some(Token::OpenSquareBracket) => {
                tokens.remove(0);
                let mut array = JSONArray::new();

                match tokens.first() {
                    Some(Token::CloseSquareBracket) => {
                        tokens.remove(0);
                        return Some(Ok(array));
                    }
                    _ => {}
                }

                while tokens.len() > 0 {
                    let json_value =
                        self.parse_value(tokens, &vec![Token::CloseSquareBracket, Token::Comma]);
                    if let Some(json_value) = json_value {
                        if let Ok(json_value) = json_value {
                            array.push(json_value);
                        } else {
                            return Some(json_err!(&json_value.unwrap_err().get_message()));
                        }
                    } else {
                        return Some(json_err!("Invalid JSON: Unexpected end of file in array"));
                    }

                    match tokens.first() {
                        Some(Token::CloseSquareBracket) => {
                            tokens.remove(0);
                            return Some(Ok(array));
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

    fn parse_object(&self, tokens: &mut Vec<Token>) -> Option<JSONResult<JSONObject>> {
        match tokens.first() {
            Some(Token::OpenCurlyBracket) => {
                tokens.remove(0);
                let mut object = JSONObject::new();

                match tokens.first() {
                    Some(Token::CloseCurlyBracket) => {
                        tokens.remove(0);
                        return Some(Ok(object));
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

                    let json_value =
                        self.parse_value(tokens, &vec![Token::CloseCurlyBracket, Token::Comma]);
                    if let Some(json_value) = json_value {
                        if let Ok(json_value) = json_value {
                            object.set(key, json_value);
                        } else {
                            return Some(json_err!(&json_value.unwrap_err().get_message()));
                        }
                    } else {
                        return Some(json_err!("Invalid JSON: Unexpected end of file in object"));
                    }

                    match tokens.first() {
                        Some(Token::CloseCurlyBracket) => {
                            tokens.remove(0);
                            return Some(Ok(object));
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
