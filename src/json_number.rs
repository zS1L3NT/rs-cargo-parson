#[derive(Debug, Clone)]
pub struct JSONNumber {
    data: f64,
}

impl JSONNumber {
    /// Create a new empty JSON Number
    pub fn new(data: f64) -> Self {
        JSONNumber { data }
    }

    /// Convert JSON Number to a Rust owned string
    ///
    /// # Example
    ///
    /// ```
    /// use parse_json::JSONNumber;
    ///
    /// assert_eq!(JSONNumber::new(0.0).to_string(), "0");
    /// assert_eq!(JSONNumber::new(1.0).to_string(), "1");
    /// assert_eq!(JSONNumber::new(-1.0).to_string(), "-1");
    /// assert_eq!(JSONNumber::new(-1e1).to_string(), "-10");
    /// assert_eq!(JSONNumber::new(-1e-1).to_string(), "-0.1");
    /// ```
    pub fn to_string(&self) -> String {
        self.data.to_string()
    }

    /// Get a Rust f64 from the JSON Number
    pub fn get_number(&self) -> f64 {
        self.data
    }
}
