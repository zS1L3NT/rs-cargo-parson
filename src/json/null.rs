use std::any::Any;

use super::JSONValue;

#[derive(Debug)]
pub struct JSONNull;

impl JSONValue for JSONNull {
    fn to_string(&self) -> String {
        format!("null")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl JSONNull {
    pub fn new() -> JSONNull {
        JSONNull
    }
}
