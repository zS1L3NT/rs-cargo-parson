use std::any::Any;

use super::JSONValue;

#[derive(Debug)]
pub struct JSONBoolean {
    data: bool,
}

impl JSONValue for JSONBoolean {
    fn to_string(&self) -> String {
        format!("{}", self.data)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl JSONBoolean {
	pub fn new(data: bool) -> JSONBoolean {
		JSONBoolean { data }
	}

    pub fn get_boolean(&self) -> bool {
        self.data
    }
}
