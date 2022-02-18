#[derive(Debug, Clone)]
pub struct JSONString {
    data: String,
}

impl JSONString {
    pub fn new(data: String) -> Self {
        JSONString { data }
    }

    pub fn to_string(&self) -> String {
        format!("\"{}\"", self.data.clone())
    }

    pub fn get_string(&self) -> String {
        self.data.clone()
    }
}
