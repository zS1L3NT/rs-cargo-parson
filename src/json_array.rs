use crate::JSONValue;

#[derive(Debug, Clone)]
pub struct JSONArray {
    data: Vec<JSONValue>,
}

impl JSONArray {
    /// Create a new empty JSON Array
    pub fn new() -> Self {
        JSONArray { data: vec![] }
    }

    /// Convert JSON Array to a Rust owned string
    ///
    /// # Example
    ///
    /// ```
    /// use parse_json::{JSONArray, JSONObject, JSONValue};
    ///
    /// let mut json_array = JSONArray::new();
    /// assert_eq!(json_array.to_string(), "[  ]");
    /// json_array.push(JSONValue::from_object(JSONObject::new()));
    /// assert_eq!(json_array.to_string(), "[ {  } ]");
    /// json_array.push(JSONValue::from_array(JSONArray::new()));
    /// assert_eq!(json_array.to_string(), "[ {  }, [  ] ]");
    /// ```
    pub fn to_string(&self) -> String {
        let mut result = "[ ".to_string();

        for value in self.data.iter() {
            result.push_str(&format!("{}, ", value.to_string()));
        }

        if self.data.len() > 0 {
            result.pop();
            result.pop();
        }

        result.push_str(" ]");
        result
    }

    /// Convert JSON Array to a Rust Vector
	pub fn to_vec(&self) -> Vec<JSONValue> {
		self.data.clone()
	}

    /// Get the number of elements in the JSON Array
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Add push JSON Value to back of the JSON Array
    pub fn push(&mut self, value: JSONValue) {
        self.data.push(value);
    }

    /// Get a value at a specific index in the JSON Array
    pub fn get(&self, index: usize) -> Option<&JSONValue> {
        self.data.get(index)
    }
}
