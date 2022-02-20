use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct JSONBoolean {
    data: bool,
}

impl Display for JSONBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl JSONBoolean {
    /// Create a new empty JSON Boolean
    pub fn new(data: bool) -> Self {
        JSONBoolean { data }
    }

    /// Convert JSON Boolean to a Rust owned string
    ///
    /// # Example
    ///
    /// ```
    /// use parson::JSONBoolean;
    ///
    /// assert_eq!(JSONBoolean::new(true).to_string(), "true");
    /// assert_eq!(JSONBoolean::new(false).to_string(), "false");
    /// ```
    pub fn to_string(&self) -> String {
        self.data.to_string()
    }

    /// Get a Rust bool from the JSON Boolean
    pub fn get_boolean(&self) -> bool {
        self.data
    }
}
