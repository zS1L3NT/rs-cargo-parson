#[derive(Debug, Clone)]
pub struct JSONString {
    data: String,
}

impl JSONString {
    /// Create a new empty JSON String
    pub fn new(data: String) -> Self {
        JSONString { data }
    }

    /// Convert JSON String to a Rust owned string
    ///
    /// # Example
    ///
    /// ```
    /// use parse_json::JSONString;
    ///
    /// assert_eq!(JSONString::new("value".to_string()).to_string(), "\"value\"");
    /// ```
    pub fn to_string(&self) -> String {
        format!("\"{}\"", self.data.clone())
    }

    /// Get a Rust owned string from the JSON String
    pub fn get_string(&self) -> String {
        self.data.clone()
    }
}
