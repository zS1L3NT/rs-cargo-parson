#[derive(Debug, Clone)]
pub struct JSONNull;

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
