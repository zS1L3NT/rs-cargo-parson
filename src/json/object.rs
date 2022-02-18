use std::collections::HashMap;

use super::value::JSONValue;

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

    pub fn get(&self, key: &str) -> Option<&JSONValue> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: JSONValue) {
        self.data.insert(key, value);
    }
}
