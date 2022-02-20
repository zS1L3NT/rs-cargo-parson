use crate::{
    json_err,
    token::{Location, Token},
    JSONArray, JSONBoolean, JSONError, JSONNull, JSONNumber, JSONObject, JSONString, JSONValue,
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<JSONValue, JSONError> {
        let mut tokens = self.tokens.clone();

        if let Some(first_token) = tokens.first() {
            match first_token {
                Token::OpenSquareBracket(location) => {
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
                Token::OpenCurlyBracket(location) => {
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
                Token::String(string, location) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_string(JSONString::new(string.clone())))
                    } else {
                        json_err!("Invalid JSON: Expected end of file after \"{}\"", string)
                    }
                }
                Token::Number(number, location) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_number(JSONNumber::new(*number)))
                    } else {
                        json_err!("Invalid JSON: Expected end of file after {}", number)
                    }
                }
                Token::Boolean(boolean, location) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_boolean(JSONBoolean::new(*boolean)))
                    } else {
                        json_err!("Invalid JSON: Expected end of file after {}", boolean)
                    }
                }
                Token::Null(location) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_null(JSONNull::new()))
                    } else {
                        json_err!("Invalid JSON: Expected end of file after null")
                    }
                }
                Token::CloseCurlyBracket(location) => {
                    json_err!("Invalid JSON: Unexpected start of file: {{")
                }
                Token::CloseSquareBracket(location) => {
                    json_err!("Invalid JSON: Unexpected start of file: }}")
                }
                Token::Colon(location) => {
                    json_err!("Invalid JSON: Unexpected start of file: :")
                }
                Token::Comma(location) => {
                    json_err!("Invalid JSON: Unexpected start of file: ,")
                }
            }
        } else {
            json_err!("Invalid JSON: No tokens found")
        }
    }

    fn parse_value(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<Result<JSONValue, JSONError>> {
        if let Some(result) = self.parse_string(tokens, &stop_tokens) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_string(results.0)))
            } else {
                json_err!(Some, result.unwrap_err().get_message())
            }
        } else if let Some(result) = self.parse_number(tokens, &stop_tokens) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_number(results.0)))
            } else {
                json_err!(Some, result.unwrap_err().get_message())
            }
        } else if let Some(result) = self.parse_boolean(tokens, &stop_tokens) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_boolean(results.0)))
            } else {
                json_err!(Some, result.unwrap_err().get_message())
            }
        } else if let Some(result) = self.parse_null(tokens, &stop_tokens) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_null(results.0)))
            } else {
                json_err!(Some, result.unwrap_err().get_message())
            }
        } else if let Some(result) = self.parse_array(tokens) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_array(results)))
            } else {
                json_err!(Some, result.unwrap_err().get_message())
            }
        } else if let Some(result) = self.parse_object(tokens) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_object(results)))
            } else {
                json_err!(Some, result.unwrap_err().get_message())
            }
        } else {
            None
        }
    }

    fn parse_string(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<Result<(JSONString, Token), JSONError>> {
        let tokens_cl = tokens.clone();
        if let Some(Token::String(string, location)) = tokens_cl.first() {
            if let Some(second_token) = tokens_cl.get(1) {
                if stop_tokens.contains(second_token) {
                    tokens.remove(0);
                    return Some(Ok((JSONString::new(string.clone()), second_token.clone())));
                }
            }
            json_err!(
                Some,
                "Invalid JSON: Unexpected end of file after string: {}",
                string
            );
        }
        None
    }

    fn parse_number(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<Result<(JSONNumber, Token), JSONError>> {
        let tokens_cl = tokens.clone();

        if let Some(Token::Number(number, location)) = tokens_cl.first() {
            if let Some(second_token) = tokens_cl.get(1) {
                if stop_tokens.contains(second_token) {
                    tokens.remove(0);
                    return Some(Ok((JSONNumber::new(*number), second_token.clone())));
                }
            }
            json_err!(
                Some,
                "Invalid JSON: Unexpected end of file after number: {}",
                number,
            );
        }
        None
    }

    fn parse_boolean(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<Result<(JSONBoolean, Token), JSONError>> {
        let tokens_cl = tokens.clone();

        if let Some(Token::Boolean(boolean, location)) = tokens_cl.first() {
            if let Some(second_token) = tokens_cl.get(1) {
                if stop_tokens.contains(second_token) {
                    tokens.remove(0);
                    return Some(Ok((JSONBoolean::new(*boolean), second_token.clone())));
                }
            }
            json_err!(
                Some,
                "Invalid JSON: Unexpected end of file after boolean: {}",
                boolean,
            );
        }
        None
    }

    fn parse_null(
        &self,
        tokens: &mut Vec<Token>,
        stop_tokens: &Vec<Token>,
    ) -> Option<Result<(JSONNull, Token), JSONError>> {
        let tokens_cl = tokens.clone();

        if let Some(Token::Null(location)) = tokens_cl.first() {
            if let Some(second_token) = tokens_cl.get(1) {
                if stop_tokens.contains(second_token) {
                    tokens.remove(0);
                    return Some(Ok((JSONNull::new(), second_token.clone())));
                }
            }
            json_err!(Some, "Invalid JSON: Unexpected end of file after null");
        }
        None
    }

    fn parse_array(&self, tokens: &mut Vec<Token>) -> Option<Result<JSONArray, JSONError>> {
        if let Some(Token::OpenSquareBracket(location)) = tokens.first() {
            tokens.remove(0);
            let mut array = JSONArray::new();

            if let Some(Token::CloseSquareBracket(location)) = tokens.first() {
                tokens.remove(0);
                return Some(Ok(array));
            }

            while tokens.len() > 0 {
                let json_value = self.parse_value(
                    tokens,
                    &vec![
                        Token::CloseSquareBracket(Location::new()),
                        Token::Comma(Location::new()),
                    ],
                );
                if let Some(json_value) = json_value {
                    if let Ok(json_value) = json_value {
                        array.push(json_value);
                    } else {
                        json_err!(Some, &json_value.unwrap_err().get_message());
                    }
                } else {
                    json_err!(Some, "Invalid JSON: Unexpected end of file in array");
                }

                match tokens.first() {
                    Some(Token::CloseSquareBracket(location)) => {
                        tokens.remove(0);
                        return Some(Ok(array));
                    }
                    Some(Token::Comma(location)) => {
                        tokens.remove(0);
                    }
                    Some(char) => {
                        json_err!(Some, "Invalid JSON: Unexpected character: {}", char)
                    }
                    _ => json_err!(Some, "Invalid JSON: Unexpected end of file"),
                }
            }

            json_err!(
                Some,
                "Invalid JSON: Unexpected end of file before finding open square bracket"
            );
        }
        None
    }

    fn parse_object(&self, tokens: &mut Vec<Token>) -> Option<Result<JSONObject, JSONError>> {
        if let Some(Token::OpenCurlyBracket(location)) = tokens.first() {
            tokens.remove(0);
            let mut object = JSONObject::new();

            if let Some(Token::CloseCurlyBracket(location)) = tokens.first() {
                tokens.remove(0);
                return Some(Ok(object));
            }

            while tokens.len() > 0 {
                let tokens_cl = tokens.clone();
                let key = tokens_cl.first().unwrap();
                let key = match key {
                    Token::String(string, location) => {
                        tokens.remove(0);
                        string.clone()
                    }
                    _ => {
                        json_err!(
                            Some,
                            "Invalid JSON: Expected string key in object, got {}",
                            key
                        );
                    }
                };

                match tokens.first() {
                    Some(Token::Colon(location)) => {
                        tokens.remove(0);
                    }
                    Some(token) => {
                        json_err!(
                            Some,
                            "Invalid JSON: Expected colon after key in object, got {}",
                            token
                        );
                    }
                    None => {
                        json_err!(Some, "Invalid JSON: Unexpected end of file")
                    }
                }

                let json_value = self.parse_value(
                    tokens,
                    &vec![
                        Token::CloseCurlyBracket(Location::new()),
                        Token::Comma(Location::new()),
                    ],
                );
                if let Some(json_value) = json_value {
                    if let Ok(json_value) = json_value {
                        object.set(key, json_value);
                    } else {
                        json_err!(Some, &json_value.unwrap_err().get_message());
                    }
                } else {
                    json_err!(Some, "Invalid JSON: Unexpected end of file in object");
                }

                match tokens.first() {
                    Some(Token::CloseCurlyBracket(location)) => {
                        tokens.remove(0);
                        return Some(Ok(object));
                    }
                    Some(Token::Comma(location)) => {
                        tokens.remove(0);
                    }
                    Some(char) => {
                        json_err!(Some, "Invalid JSON: Unexpected character: {}", char)
                    }
                    _ => {
                        json_err!(Some, "Invalid JSON: Unexpected end of file")
                    }
                }
            }

            json_err!(
                Some,
                "Invalid JSON: Unexpected end of file before finding open curly bracket"
            );
        }
        None
    }
}
