use crate::{
    json_err,
    token::{Token, TokenType},
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

        if let Some(token) = tokens.first() {
            let token = token.clone();

            match &token.token_type {
                TokenType::OpenSquareBracket => {
                    let json_array = self.parse_array(&mut tokens);
                    if let Some(json_array) = json_array {
                        if let Ok(json_array) = json_array {
                            Ok(JSONValue::from_array(json_array))
                        } else {
                            json_err!(json_array.unwrap_err())
                        }
                    } else {
                        json_err!("First token of array not <[>"; token.line, token.column)
                    }
                }
                TokenType::OpenCurlyBracket => {
                    let json_object = self.parse_object(&mut tokens);
                    if let Some(json_object) = json_object {
                        if let Ok(json_object) = json_object {
                            Ok(JSONValue::from_object(json_object))
                        } else {
                            json_err!(json_object.unwrap_err())
                        }
                    } else {
                        json_err!("First token of object not <{>"; token.line, token.column)
                    }
                }
                TokenType::String(string) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_string(JSONString::new(string.clone())))
                    } else {
                        json_err!("Expected end of file, got <{}>", string; token.line, token.column)
                    }
                }
                TokenType::Number(number) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_number(JSONNumber::new(*number)))
                    } else {
                        json_err!("Expected end of file, got <{}>", number; token.line, token.column)
                    }
                }
                TokenType::Boolean(boolean) => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_boolean(JSONBoolean::new(*boolean)))
                    } else {
                        json_err!("Expected end of file, got <{}>", boolean; token.line, token.column)
                    }
                }
                TokenType::Null => {
                    if tokens.len() == 1 {
                        Ok(JSONValue::from_null(JSONNull::new()))
                    } else {
                        json_err!(
                            "Expected end of file, got <null>";
                            token.line,
                            token.column
                        )
                    }
                }
                TokenType::CloseCurlyBracket => {
                    json_err!("Unexpected start of file <{>"; token.line, token.column)
                }
                TokenType::CloseSquareBracket => {
                    json_err!("Unexpected start of file <}>"; token.line, token.column)
                }
                TokenType::Colon => {
                    json_err!("Unexpected start of file <:>"; token.line, token.column)
                }
                TokenType::Comma => {
                    json_err!("Unexpected start of file <,>"; token.line, token.column)
                }
            }
        } else {
            json_err!("No tokens found"; 1, 1)
        }
    }

    fn parse_value(
        &self,
        tokens: &mut Vec<Token>,
        stop_token_types: &Vec<TokenType>,
    ) -> Option<Result<JSONValue, JSONError>> {
        if let Some(result) = self.parse_string(tokens, &stop_token_types) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_string(results.0)))
            } else {
                json_err!(Some; result.unwrap_err())
            }
        } else if let Some(result) = self.parse_number(tokens, &stop_token_types) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_number(results.0)))
            } else {
                json_err!(Some; result.unwrap_err())
            }
        } else if let Some(result) = self.parse_boolean(tokens, &stop_token_types) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_boolean(results.0)))
            } else {
                json_err!(Some; result.unwrap_err())
            }
        } else if let Some(result) = self.parse_null(tokens, &stop_token_types) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_null(results.0)))
            } else {
                json_err!(Some; result.unwrap_err())
            }
        } else if let Some(result) = self.parse_array(tokens) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_array(results)))
            } else {
                json_err!(Some; result.unwrap_err())
            }
        } else if let Some(result) = self.parse_object(tokens) {
            if let Ok(results) = result {
                Some(Ok(JSONValue::from_object(results)))
            } else {
                json_err!(Some; result.unwrap_err())
            }
        } else {
            None
        }
    }

    fn parse_string(
        &self,
        tokens: &mut Vec<Token>,
        stop_token_types: &Vec<TokenType>,
    ) -> Option<Result<(JSONString, Token), JSONError>> {
        let tokens_cl = tokens.clone();
        if let Some(token) = tokens_cl.first() {
            if let TokenType::String(string) = &token.token_type {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_token_types.contains(&second_token.token_type) {
                        tokens.remove(0);
                        return Some(Ok((JSONString::new(string.clone()), second_token.clone())));
                    }
                }
                json_err!(
                    Some;
                    "Unexpected end of file <{}>",
                    string;
                    token.line,
                    token.column
                )
            }
        }
        None
    }

    fn parse_number(
        &self,
        tokens: &mut Vec<Token>,
        stop_token_types: &Vec<TokenType>,
    ) -> Option<Result<(JSONNumber, Token), JSONError>> {
        let tokens_cl = tokens.clone();
        if let Some(token) = tokens_cl.first() {
            if let TokenType::Number(number) = token.token_type {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_token_types.contains(&second_token.token_type) {
                        tokens.remove(0);
                        return Some(Ok((JSONNumber::new(number), second_token.clone())));
                    }
                }
                json_err!(
                    Some;
                    "Unexpected end of file <{}>",
                    number;
                    token.line,
                    token.column
                )
            }
        }
        None
    }

    fn parse_boolean(
        &self,
        tokens: &mut Vec<Token>,
        stop_token_types: &Vec<TokenType>,
    ) -> Option<Result<(JSONBoolean, Token), JSONError>> {
        let tokens_cl = tokens.clone();

        if let Some(token) = tokens_cl.first() {
            if let TokenType::Boolean(boolean) = token.token_type {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_token_types.contains(&second_token.token_type) {
                        tokens.remove(0);
                        return Some(Ok((JSONBoolean::new(boolean), second_token.clone())));
                    }
                }
                json_err!(
                    Some;
                    "Unexpected end of file <{}>",
                    boolean;
                    token.line,
                    token.column
                )
            }
        }
        None
    }

    fn parse_null(
        &self,
        tokens: &mut Vec<Token>,
        stop_token_types: &Vec<TokenType>,
    ) -> Option<Result<(JSONNull, Token), JSONError>> {
        let tokens_cl = tokens.clone();

        if let Some(token) = tokens_cl.first() {
            if let TokenType::Null = token.token_type {
                if let Some(second_token) = tokens_cl.get(1) {
                    if stop_token_types.contains(&second_token.token_type) {
                        tokens.remove(0);
                        return Some(Ok((JSONNull::new(), second_token.clone())));
                    }
                }
                json_err!(Some; "Unexpected end of file <null>"; token.line, token.column)
            }
        }
        None
    }

    fn parse_array(&self, tokens: &mut Vec<Token>) -> Option<Result<JSONArray, JSONError>> {
        if let Some(token) = tokens.first() {
            let token = token.clone();
            if let TokenType::OpenSquareBracket = token.token_type {
                tokens.remove(0);
                let mut array = JSONArray::new();

                if let Some(token) = tokens.first() {
                    if let TokenType::CloseSquareBracket = token.token_type {
                        tokens.remove(0);
                        return Some(Ok(array));
                    }
                }

                while tokens.len() > 0 {
                    let json_value = self.parse_value(
                        tokens,
                        &vec![TokenType::CloseSquareBracket, TokenType::Comma],
                    );
                    if let Some(json_value) = json_value {
                        if let Ok(json_value) = json_value {
                            array.push(json_value);
                        } else {
                            json_err!(Some; json_value.unwrap_err())
                        }
                    } else {
                        json_err!(Some; "Unexpected end of file"; token.line, token.column)
                    }

                    if let Some(token) = tokens.first() {
                        match &token.token_type {
                            TokenType::CloseSquareBracket => {
                                tokens.remove(0);
                                return Some(Ok(array));
                            }
                            TokenType::Comma => {
                                tokens.remove(0);
                            }
                            char => {
                                json_err!(Some; "Unexpected character <{}>", char; token.line, token.column)
                            }
                        }
                    } else {
                        json_err!(Some; "Unexpected end of file"; token.line, token.column)
                    }
                }

                json_err!(
                    Some;
                    "Unexpected end of file before finding open square bracket"; token.line, token.column
                )
            }
        }
        None
    }

    fn parse_object(&self, tokens: &mut Vec<Token>) -> Option<Result<JSONObject, JSONError>> {
        if let Some(token) = tokens.first() {
            let token = token.clone();
            if let TokenType::OpenCurlyBracket = token.token_type {
                tokens.remove(0);
                let mut object = JSONObject::new();

                if let Some(token) = tokens.first() {
                    if let TokenType::CloseCurlyBracket = token.token_type {
                        tokens.remove(0);
                        return Some(Ok(object));
                    }
                }

                while tokens.len() > 0 {
                    let tokens_cl = tokens.clone();
                    let token = tokens_cl.first().unwrap();
                    let key = if let TokenType::String(string) = &token.token_type {
                        tokens.remove(0);
                        string.clone()
                    } else {
                        json_err!(
                            Some;
                            "Expected string key in object, got <{}>",
                            token.token_type;
                            token.line, token.column
                        )
                    };

                    if let Some(token) = tokens.first() {
                        if let TokenType::Colon = token.token_type {
                            tokens.remove(0);
                        } else {
                            json_err!(
                                Some;
                                "Expected colon after key in object, got <{}>",
                                token.token_type; token.line, token.column
                            )
                        }
                    } else {
                        json_err!(Some; "Unexpected end of file"; token.line, token.column)
                    }

                    let json_value = self.parse_value(
                        tokens,
                        &vec![TokenType::CloseCurlyBracket, TokenType::Comma],
                    );
                    if let Some(json_value) = json_value {
                        if let Ok(json_value) = json_value {
                            object.set(key, json_value);
                        } else {
                            json_err!(Some; json_value.unwrap_err())
                        }
                    } else {
                        json_err!(Some; "Unexpected end of file"; token.line, token.column);
                    }

                    if let Some(token) = tokens.first() {
                        match &token.token_type {
                            TokenType::CloseCurlyBracket => {
                                tokens.remove(0);
                                return Some(Ok(object));
                            }
                            TokenType::Comma => {
                                tokens.remove(0);
                            }
                            char => {
                                json_err!(Some; "Unexpected character <{}>", char; token.line, token.column)
                            }
                        }
                    } else {
                        json_err!(Some; "Unexpected end of file"; token.line, token.column)
                    }
                }

                json_err!(
                    Some;
                    "Unexpected end of file before finding open curly bracket";
                    token.line, token.column
                );
            }
        }
        None
    }
}
