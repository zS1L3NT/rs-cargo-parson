use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct JSONString {
    data: String,
}

impl Display for JSONString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
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
    /// use parson::JSONString;
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
