use std::any::Any;

use super::JSONValue;

#[derive(Debug)]
pub struct JSONString {
    data: String,
}

impl JSONValue for JSONString {
    fn to_string(&self) -> String {
        format!("\"{}\"", self.data.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl JSONString {
    pub fn new(data: String) -> Self {
        JSONString { data }
    }

    pub fn get_string(&self) -> String {
        self.data.clone()
    }
}
