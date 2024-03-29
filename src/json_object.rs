use std::{collections::HashMap, fmt::Display};

use indexmap::{indexmap, IndexMap};

use crate::JSONValue;

#[derive(Debug, Clone)]
pub struct JSONObject {
    data: IndexMap<String, JSONValue>,
}

impl Display for JSONObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl JSONObject {
    /// Create a new empty JSON Object
    pub fn new() -> Self {
        JSONObject { data: indexmap! {} }
    }

    /// Convert JSON Object to a Rust owned string
    ///
    /// # Example
    ///
    /// ```
    /// use parson::{JSONNumber, JSONObject, JSONString, JSONValue};
    ///
    /// let mut json_object = JSONObject::new();
    /// assert_eq!(json_object.to_string(), "{  }");
    /// json_object.set("key".to_string(), JSONValue::from_string(JSONString::new("value".to_string())));
    /// assert_eq!(json_object.to_string(), "{ \"key\": \"value\" }");
    /// json_object.set("number".to_string(), JSONValue::from_number(JSONNumber::new(1.0)));
    /// assert_eq!(json_object.to_string(), "{ \"key\": \"value\", \"number\": 1 }");
    /// ```
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

	/// Format JSON Object to a Rust owned string
    pub fn format_string(&self, indents: i32, spaces: i32) -> String {
        if self.data.len() == 0 {
            return String::from("{}");
        }

        let mut result = "{".to_string();

        for (key, value) in self.data.iter() {
            result.push_str(&format!(
                "\n{}\"{}\": {},",
                " ".repeat(((indents + 1) * spaces) as usize),
                key,
                value.format_string(indents + 1, spaces)
            ));
        }

        result.push_str(&format!("\n{}}}", " ".repeat((indents * spaces) as usize)));
        result
    }

    /// Convert JSON Object to a Rust HashMap
    pub fn to_hashmap(&self) -> HashMap<String, JSONValue> {
        self.data
            .keys()
            .map(|k| (k.clone(), self.data[k].clone()))
            .collect()
    }

    /// Get the number of key-value pairs in the JSON Object
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get a value by a specific key in the JSON Object
    pub fn get(&self, key: &str) -> Option<&JSONValue> {
        self.data.get(key)
    }

    /// Add a key value pair to the JSON Object
    pub fn set(&mut self, key: String, value: JSONValue) {
        self.data.insert(key, value);
    }
}
