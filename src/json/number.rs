use std::any::Any;

use super::JSONValue;

#[derive(Debug)]
pub struct JSONNumber {
    data: f64,
}

impl JSONValue for JSONNumber {
    fn to_string(&self) -> String {
        format!("{}", self.data)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl JSONNumber {
	pub fn new(data: f64) -> JSONNumber {
		JSONNumber { data }
	}

    pub fn get_number(&self) -> f64 {
        self.data
    }
}
