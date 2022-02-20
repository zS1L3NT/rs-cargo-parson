use std::fmt::Display;

#[derive(Debug)]
pub struct JSONError {
    message: String,
}

impl Display for JSONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
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
		return Err($crate::json_error::JSONError::new($arg))
	};
	(Some, $arg:expr) => {
		return Some(Err($crate::json_error::JSONError::new($arg)))
	};
	(Some, $($args:tt)*) => {
		return Some(Err($crate::json_error::JSONError::new(&format!($($args)*))))
	};
	($($args:tt)*) => {
		return Err($crate::json_error::JSONError::new(&format!($($args)*)))
	}
}
