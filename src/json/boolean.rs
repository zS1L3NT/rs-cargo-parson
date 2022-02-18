#[derive(Debug, Clone)]
pub struct JSONBoolean {
    data: bool,
}

impl JSONBoolean {
    pub fn new(data: bool) -> Self {
        JSONBoolean { data }
    }

    pub fn to_string(&self) -> String {
        self.data.to_string()
    }

    pub fn get_boolean(&self) -> bool {
        self.data
    }
}
