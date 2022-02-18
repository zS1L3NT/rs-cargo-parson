use super::JSONValue;

#[derive(Debug)]
pub struct JSONNull;

impl JSONValue for JSONNull {
    fn to_string(&self) -> String {
        "null".to_string()
    }
}

impl JSONNull {
    pub fn new() -> Self {
        JSONNull
    }
}
