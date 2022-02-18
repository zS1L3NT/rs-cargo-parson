#[derive(Debug, Clone)]
pub struct JSONNumber {
    data: f64,
}

impl JSONNumber {
    pub fn new(data: f64) -> Self {
        JSONNumber { data }
    }

    pub fn to_string(&self) -> String {
        self.data.to_string()
    }

    pub fn get_number(&self) -> f64 {
        self.data
    }
}
