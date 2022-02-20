use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct JSONNull;

impl Display for JSONNull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl JSONNull {
    /// Create a new empty JSON Null
    pub fn new() -> Self {
        JSONNull {}
    }

    /// Convert JSON Null to a Rust owned string
    ///
    /// # Example
    ///
    /// ```
    /// use parson::JSONNull;
    ///
    /// assert_eq!(JSONNull::new().to_string(), "null");
    /// ```
    pub fn to_string(&self) -> String {
        "null".to_string()
    }
}
