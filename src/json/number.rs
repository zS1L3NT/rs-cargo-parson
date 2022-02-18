use super::JSONValue;

#[derive(Debug)]
pub struct JSONNumber {
    data: f64,
}

impl JSONValue for JSONNumber {
    fn to_string(&self) -> String {
        self.data.to_string()
    }
}

impl JSONNumber {
    pub fn new(data: f64) -> Self {
        JSONNumber { data }
    }

    pub fn get_number(&self) -> f64 {
        self.data
    }
}
