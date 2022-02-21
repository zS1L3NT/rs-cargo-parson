use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{
    json_err, json_error::JSONError, lexer::Lexer, parser::Parser, JSONArray, JSONBoolean,
    JSONNull, JSONNumber, JSONObject, JSONString,
};

#[derive(Debug, Clone)]
pub enum JSONType {
    String(JSONString),
    Number(JSONNumber),
    Boolean(JSONBoolean),
    Null(JSONNull),
    Array(JSONArray),
    Object(JSONObject),
}

#[derive(Debug, Clone)]
pub struct JSONValue {
    data: JSONType,
}

impl FromStr for JSONValue {
    type Err = JSONError;

    fn from_str(json: &str) -> Result<Self, Self::Err> {
        let lexer = Lexer::new(json.to_string());
        let tokens = lexer.get_tokens();
        if let Ok(tokens) = tokens {
            let parser = Parser::new(tokens);
            let json_value = parser.parse();
            if let Ok(json_value) = json_value {
                Ok(json_value)
            } else {
                json_err!(json_value.unwrap_err())
            }
        } else {
            json_err!(tokens.unwrap_err())
        }
    }
}

impl Display for JSONValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl JSONValue {
    /// Create a new JSON Value from a JSON String
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONString, JSONValue};
    ///
    /// let json_string = JSONString::new(String::new());
    /// let json_value = JSONValue::from_string(json_string);
    /// ```
    pub fn from_string(json_string: JSONString) -> Self {
        JSONValue {
            data: JSONType::String(json_string),
        }
    }

    /// Create a new JSON Value from a JSON Number
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONNumber, JSONValue};
    ///
    /// let json_number = JSONNumber::new(1.0);
    /// let json_value = JSONValue::from_number(json_number);
    /// ```
    pub fn from_number(json_number: JSONNumber) -> Self {
        JSONValue {
            data: JSONType::Number(json_number),
        }
    }

    /// Create a new JSON Value from a JSON Boolean
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONValue, JSONBoolean};
    ///
    /// let json_boolean = JSONBoolean::new(true);
    /// let json_value = JSONValue::from_boolean(json_boolean);
    /// ```
    pub fn from_boolean(json_boolean: JSONBoolean) -> Self {
        JSONValue {
            data: JSONType::Boolean(json_boolean),
        }
    }

    /// Create a new JSON Value from a JSON Null
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONNull, JSONValue};
    ///
    /// let json_null = JSONNull::new();
    /// let json_value = JSONValue::from_null(json_null);
    /// ```
    pub fn from_null(json_null: JSONNull) -> Self {
        JSONValue {
            data: JSONType::Null(json_null),
        }
    }

    /// Create a new JSON Value from a JSON Array
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONArray, JSONValue};
    ///
    /// let json_array = JSONArray::new();
    /// let json_value = JSONValue::from_array(json_array);
    /// ```
    pub fn from_array(json_array: JSONArray) -> Self {
        JSONValue {
            data: JSONType::Array(json_array),
        }
    }

    /// Create a new JSON Value from a JSON Object
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONObject, JSONValue};
    ///
    /// let json_object = JSONObject::new();
    /// let json_value = JSONValue::from_object(json_object);
    /// ```
    pub fn from_object(json_object: JSONObject) -> Self {
        JSONValue {
            data: JSONType::Object(json_object),
        }
    }

    /// Get the type of the JSON Value
    ///
    /// # Example
    ///
    /// ```
    ///	use parson::{JSONNull, JSONType, JSONValue};
    ///
    /// let json_value = JSONValue::from_null(JSONNull::new());
    /// assert!(match json_value.get_type() {
    /// 	JSONType::Null(json_null) => true,
    /// 	_ => false,
    /// });
    /// ```
    pub fn get_type(&self) -> JSONType {
        self.data.clone()
    }

    /// Cast the JSON Value to a Rust owned string
    pub fn get_string(&self) -> Result<String, JSONError> {
        match &self.data {
            JSONType::String(json_string) => Ok(json_string.get_string()),
            _ => json_err!("JSONValue::get_string() called on non-string value"; 0, 0),
        }
    }

    /// Cast the JSON Value to a Rust f64
    pub fn get_number(&self) -> Result<f64, JSONError> {
        match &self.data {
            JSONType::Number(json_number) => Ok(json_number.get_number()),
            _ => json_err!("JSONValue::get_number() called on non-number value"; 0, 0),
        }
    }

    /// Cast the JSON Value to a Rust bool
    pub fn get_boolean(&self) -> Result<bool, JSONError> {
        match &self.data {
            JSONType::Boolean(json_boolean) => Ok(json_boolean.get_boolean()),
            _ => json_err!("JSONValue::get_boolean() called on non-boolean value"; 0, 0),
        }
    }

    /// Cast the JSON Value to JSON Null
    pub fn get_null(&self) -> Result<&JSONNull, JSONError> {
        match &self.data {
            JSONType::Null(json_null) => Ok(json_null),
            _ => json_err!("JSONValue::get_null() called on non-null value"; 0, 0),
        }
    }

    /// Cast the JSON Value to JSON Array
    pub fn get_array(&self) -> Result<&JSONArray, JSONError> {
        match &self.data {
            JSONType::Array(json_array) => Ok(json_array),
            _ => json_err!("JSONValue::get_array() called on non-array value"; 0, 0),
        }
    }

    /// Cast the JSON Value to JSON Object
    pub fn get_object(&self) -> Result<&JSONObject, JSONError> {
        match &self.data {
            JSONType::Object(json_object) => Ok(json_object),
            _ => json_err!("JSONValue::get_object() called on non-object value"; 0, 0),
        }
    }

    /// If the JSON Value is a JSON String
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONString, JSONValue};
    ///
    /// let json_string = JSONString::new(String::new());
    /// let json_value = JSONValue::from_string(json_string);
    /// assert!(json_value.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        match self.data {
            JSONType::String(_) => true,
            _ => false,
        }
    }

    /// If the JSON Value is a JSON Number
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONNumber, JSONValue};
    ///
    /// let json_number = JSONNumber::new(0.0);
    /// let json_value = JSONValue::from_number(json_number);
    /// assert!(json_value.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        match self.data {
            JSONType::Number(_) => true,
            _ => false,
        }
    }

    /// If the JSON Value is a JSON Boolean
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONBoolean, JSONValue};
    ///
    /// let json_boolean = JSONBoolean::new(true);
    /// let json_value = JSONValue::from_boolean(json_boolean);
    /// assert!(json_value.is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        match self.data {
            JSONType::Boolean(_) => true,
            _ => false,
        }
    }

    /// If the JSON Value is a JSON Null
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONNull, JSONValue};
    ///
    /// let json_null = JSONNull::new();
    /// let json_value = JSONValue::from_null(json_null);
    /// assert!(json_value.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        match self.data {
            JSONType::Null(_) => true,
            _ => false,
        }
    }

    /// If the JSON Value is a JSON Array
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONArray, JSONValue};
    ///
    /// let json_array = JSONArray::new();
    /// let json_value = JSONValue::from_array(json_array);
    /// assert!(json_value.is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        match self.data {
            JSONType::Array(_) => true,
            _ => false,
        }
    }

    /// If the JSON Value is a JSON Object
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONObject, JSONValue};
    ///
    /// let json_object = JSONObject::new();
    /// let json_value = JSONValue::from_object(json_object);
    /// assert!(json_value.is_object());
    /// ```
    pub fn is_object(&self) -> bool {
        match self.data {
            JSONType::Object(_) => true,
            _ => false,
        }
    }

    /// Convert JSON Value to a Rust owned string
    pub fn to_string(&self) -> String {
        match &self.data {
            JSONType::String(json_string) => json_string.to_string(),
            JSONType::Number(json_number) => json_number.to_string(),
            JSONType::Boolean(json_boolean) => json_boolean.to_string(),
            JSONType::Null(json_null) => json_null.to_string(),
            JSONType::Array(json_array) => json_array.to_string(),
            JSONType::Object(json_object) => json_object.to_string(),
        }
    }
}
