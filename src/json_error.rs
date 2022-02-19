#[derive(Debug)]
pub struct JSONError {
    message: String,
}

impl JSONError {
    pub fn new(message: &str) -> Self {
        JSONError {
            message: message.to_string(),
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message[..]
    }
}

#[macro_export]
macro_rules! json_err {
	($arg:expr) => {
		Err($crate::json_error::JSONError::new($arg))
	};
	($($args:tt)*) => {
		Err($crate::json_error::JSONError::new(&format!($($args)*)))
	}
}
