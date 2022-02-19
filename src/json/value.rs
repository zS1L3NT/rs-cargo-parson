use std::fmt::Debug;

use crate::{JSONArray, JSONBoolean, JSONNull, JSONNumber, JSONObject, JSONString};

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

impl JSONValue {
    pub fn from_string(json_string: JSONString) -> Self {
        JSONValue {
            data: JSONType::String(json_string),
        }
    }

    pub fn from_number(json_number: JSONNumber) -> Self {
        JSONValue {
            data: JSONType::Number(json_number),
        }
    }

    pub fn from_boolean(json_boolean: JSONBoolean) -> Self {
        JSONValue {
            data: JSONType::Boolean(json_boolean),
        }
    }

    pub fn from_null(json_null: JSONNull) -> Self {
        JSONValue {
            data: JSONType::Null(json_null),
        }
    }

    pub fn from_array(json_array: JSONArray) -> Self {
        JSONValue {
            data: JSONType::Array(json_array),
        }
    }

    pub fn from_object(json_object: JSONObject) -> Self {
        JSONValue {
            data: JSONType::Object(json_object),
        }
    }

    pub fn get_type(&self) -> JSONType {
        self.data.clone()
    }

    pub fn get_string(&self) -> String {
        match &self.data {
            JSONType::String(json_string) => json_string.get_string(),
            _ => panic!("JSONValue::get_string() called on non-string value"),
        }
    }

    pub fn get_number(&self) -> f64 {
        match &self.data {
            JSONType::Number(json_number) => json_number.get_number(),
            _ => panic!("JSONValue::get_number() called on non-number value"),
        }
    }

    pub fn get_boolean(&self) -> bool {
        match &self.data {
            JSONType::Boolean(json_boolean) => json_boolean.get_boolean(),
            _ => panic!("JSONValue::get_boolean() called on non-boolean value"),
        }
    }

    pub fn get_null(&self) -> &JSONNull {
        match &self.data {
            JSONType::Null(json_null) => &json_null,
            _ => panic!("JSONValue::get_null() called on non-null value"),
        }
    }

    pub fn get_array(&self) -> &JSONArray {
        match &self.data {
            JSONType::Array(json_array) => &json_array,
            _ => panic!("JSONValue::get_array() called on non-array value"),
        }
    }

    pub fn get_object(&self) -> &JSONObject {
        match &self.data {
            JSONType::Object(json_object) => &json_object,
            _ => panic!("JSONValue::get_object() called on non-object value"),
        }
    }

    pub fn is_string(&self) -> bool {
        match self.data {
            JSONType::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self.data {
            JSONType::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self.data {
            JSONType::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self.data {
            JSONType::Null(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self.data {
            JSONType::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self.data {
            JSONType::Object(_) => true,
            _ => false,
        }
    }

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
