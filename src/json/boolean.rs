use super::JSONValue;

#[derive(Debug)]
pub struct JSONBoolean {
    data: bool,
}

impl JSONValue for JSONBoolean {
    fn to_string(&self) -> String {
        self.data.to_string()
    }
}

impl JSONBoolean {
    pub fn new(data: bool) -> Self {
        JSONBoolean { data }
    }

    pub fn get_boolean(&self) -> bool {
        self.data
    }
}
