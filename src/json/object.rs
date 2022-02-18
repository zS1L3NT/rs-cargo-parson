use std::collections::HashMap;

use super::{
    array::JSONArray, boolean::JSONBoolean, null::JSONNull, number::JSONNumber, string::JSONString,
    JSONValue,
};

#[derive(Debug)]
pub struct JSONObject {
    data: HashMap<String, Box<dyn JSONValue>>,
}

impl JSONValue for JSONObject {
    fn to_string(&self) -> String {
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
}

impl JSONObject {
    pub fn new() -> Self {
        JSONObject {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Box<dyn JSONValue>) {
        self.data.insert(key, value);
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is::<JSONString>() {
            panic!("Value for key is not of type String: {}", key);
        }

        Some(
            self.data[key]
                .downcast_ref::<JSONString>()
                .unwrap()
                .get_string(),
        )
    }

    pub fn get_number(&self, key: &str) -> Option<f64> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is::<JSONNumber>() {
            panic!("Value for key is not of type Number: {}", key);
        }

        Some(
            self.data[key]
                .downcast_ref::<JSONNumber>()
                .unwrap()
                .get_number(),
        )
    }

    pub fn get_boolean(&self, key: &str) -> Option<bool> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is::<JSONBoolean>() {
            panic!("Value for key is not of type Boolean: {}", key);
        }

        Some(
            self.data[key]
                .downcast_ref::<JSONBoolean>()
                .unwrap()
                .get_boolean(),
        )
    }

    pub fn get_null(&self, key: &str) -> Option<&JSONNull> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is::<JSONNull>() {
            panic!("Value for key is not of type Null: {}", key);
        }

        Some(self.data[key].downcast_ref::<JSONNull>().unwrap())
    }

    pub fn get_array(&self, key: &str) -> Option<&JSONArray> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is::<JSONArray>() {
            panic!("Value for key is not of type Array: {}", key);
        }

        Some(self.data[key].downcast_ref::<JSONArray>().unwrap())
    }

    pub fn get_object(&self, key: &str) -> Option<&JSONObject> {
        if self.data.get(key).is_none() {
            return None;
        }

        if !self.data[key].is::<JSONObject>() {
            panic!("Value for key is not of type Object: {}", key);
        }

        Some(self.data[key].downcast_ref::<JSONObject>().unwrap())
    }
}
