use std::collections::HashMap;

use super::{array::JSONArray, null::JSONNull, value::JSONValue};

#[derive(Debug, Clone)]
pub struct JSONObject {
    data: HashMap<String, JSONValue>,
}

impl JSONObject {
    pub fn new() -> Self {
        JSONObject {
            data: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = "{ ".to_string();

        for (key, value) in self.data.iter() {
            result.push_str(&format!("\"{}\": {}, ", key, value.to_string()));
        }

        if self.data.len() > 0 {
            result.pop();
            result.pop();
        }

        result.push_str(" }");
        result
    }

    pub fn set(&mut self, key: String, value: JSONValue) {
        self.data.insert(key, value);
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is_string() {
            panic!("Value for key is not of type String: {}", key);
        }

        Some(self.data[key].get_string())
    }

    pub fn get_number(&self, key: &str) -> Option<f64> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is_number() {
            panic!("Value for key is not of type Number: {}", key);
        }

        Some(self.data[key].get_number())
    }

    pub fn get_boolean(&self, key: &str) -> Option<bool> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is_boolean() {
            panic!("Value for key is not of type Boolean: {}", key);
        }

        Some(self.data[key].get_boolean())
    }

    pub fn get_null(&self, key: &str) -> Option<&JSONNull> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is_null() {
            panic!("Value for key is not of type Null: {}", key);
        }

        Some(self.data[key].get_null())
    }

    pub fn get_array(&self, key: &str) -> Option<&JSONArray> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is_array() {
            panic!("Value for key is not of type Array: {}", key);
        }

        Some(self.data[key].get_array())
    }

    pub fn get_object(&self, key: &str) -> Option<&JSONObject> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is_object() {
            panic!("Value for key is not of type Object: {}", key);
        }

        Some(self.data[key].get_object())
    }
}
