use super::{null::JSONNull, object::JSONObject, value::JSONValue};

#[derive(Debug, Clone)]
pub struct JSONArray {
    data: Vec<JSONValue>,
}

impl JSONArray {
    pub fn new() -> Self {
        JSONArray { data: vec![] }
    }

    pub fn to_string(&self) -> String {
        let mut result = "[ ".to_string();

        for value in self.data.iter() {
            result.push_str(&format!("{}, ", value.to_string()));
        }

        if self.data.len() > 0 {
            result.pop();
            result.pop();
        }

        result.push_str(" ]");
        result
    }

    pub fn push(&mut self, value: JSONValue) {
        self.data.push(value);
    }

    pub fn get_string(&self, index: usize) -> Option<String> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is_string() {
            panic!("Value at index is not of type String: {}", index);
        }

        Some(self.data[index].get_string())
    }

    pub fn get_number(&self, index: usize) -> Option<f64> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is_number() {
            panic!("Value at index is not of type Number: {}", index);
        }

        Some(self.data[index].get_number())
    }

    pub fn get_boolean(&self, index: usize) -> Option<bool> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is_boolean() {
            panic!("Value at index is not of type Boolean: {}", index);
        }

        Some(self.data[index].get_boolean())
    }

    pub fn get_null(&self, index: usize) -> Option<&JSONNull> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is_null() {
            panic!("Value at index is not of type Null: {}", index);
        }

        Some(self.data[index].get_null())
    }

    pub fn get_array(&self, index: usize) -> Option<&JSONArray> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is_array() {
            panic!("Value at index is not of type Array: {}", index);
        }

        Some(self.data[index].get_array())
    }

    pub fn get_object(&self, index: usize) -> Option<&JSONObject> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is_object() {
            panic!("Value at index is not of type Object: {}", index);
        }

        Some(self.data[index].get_object())
    }

    pub fn get_value(&self, index: usize) -> Option<&JSONValue> {
        self.data.get(index)
    }
}
