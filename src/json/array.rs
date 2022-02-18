use super::{
    boolean::JSONBoolean, null::JSONNull, number::JSONNumber, object::JSONObject,
    string::JSONString, JSONValue,
};

#[derive(Debug)]
pub struct JSONArray {
    data: Vec<Box<dyn JSONValue>>,
}

impl JSONValue for JSONArray {
    fn to_string(&self) -> String {
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
}

impl JSONArray {
    pub fn new() -> Self {
        JSONArray { data: vec![] }
    }

    pub fn push(&mut self, value: Box<dyn JSONValue>) {
        self.data.push(value);
    }

    pub fn get_string(&self, index: usize) -> Option<String> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is::<JSONString>() {
            panic!("Value at index is not of type String: {}", index);
        }

        Some(
            self.data[index]
                .downcast_ref::<JSONString>()
                .unwrap()
                .get_string(),
        )
    }

    pub fn get_number(&self, index: usize) -> Option<f64> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is::<JSONNumber>() {
            panic!("Value at index is not of type Number: {}", index);
        }

        Some(
            self.data[index]
                .downcast_ref::<JSONNumber>()
                .unwrap()
                .get_number(),
        )
    }

    pub fn get_boolean(&self, index: usize) -> Option<bool> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is::<JSONBoolean>() {
            panic!("Value at index is not of type Boolean: {}", index);
        }

        Some(
            self.data[index]
                .downcast_ref::<JSONBoolean>()
                .unwrap()
                .get_boolean(),
        )
    }

    pub fn get_null(&self, index: usize) -> Option<&JSONNull> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is::<JSONNull>() {
            panic!("Value at index is not of type Null: {}", index);
        }

        Some(self.data[index].downcast_ref::<JSONNull>().unwrap())
    }

    pub fn get_array(&self, index: usize) -> Option<&JSONArray> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is::<JSONArray>() {
            panic!("Value at index is not of type Array: {}", index);
        }

        Some(self.data[index].downcast_ref::<JSONArray>().unwrap())
    }

    pub fn get_object(&self, index: usize) -> Option<&JSONObject> {
        if self.data.get(index).is_none() {
            return None;
        }

        if !self.data[index].is::<JSONObject>() {
            panic!("Value at index is not of type Object: {}", index);
        }

        Some(self.data[index].downcast_ref::<JSONObject>().unwrap())
    }

    pub fn get_value(&self, index: usize) -> Option<&Box<dyn JSONValue>> {
        self.data.get(index)
    }
}
