#[derive(Debug, Clone)]
pub struct JSONNull;

impl JSONNull {
    pub fn new() -> Self {
        JSONNull {}
    }

    pub fn to_string(&self) -> String {
        "null".to_string()
    }
}
