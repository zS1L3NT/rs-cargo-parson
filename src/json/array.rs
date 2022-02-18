use super::value::JSONValue;

#[derive(Debug, Clone)]
pub struct JSONArray {
    data: Vec<JSONValue>,
}

impl JSONArray {
    pub fn new() -> Self {
        JSONArray { data: vec![] }
    }

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

    pub fn push(&mut self, value: JSONValue) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&JSONValue> {
        self.data.get(index)
    }
}
