use std::fmt::{Debug, Display};

pub struct JSONError {
    message: String,
    line: usize,
    column: usize,
}

impl Debug for JSONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

impl Display for JSONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

impl JSONError {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        JSONError {
            message: message.to_string(),
            line,
            column,
        }
    }

    pub fn get_message(&self) -> String {
        format!(
            "{} (line {} column {})",
            self.message, self.line, self.column
        )
    }
}

#[macro_export]
macro_rules! json_err {
	(Some; $error:expr) => {
		return Some(Err($error))
	};
	($error:expr) => {
		return Err($error)
	};
	(Some; $message:expr; $line:expr, $column:expr) => {
		return Some(Err($crate::json_error::JSONError::new($message, $line, $column)))
	};
	($message:expr; $line:expr, $column:expr) => {
		return Err($crate::json_error::JSONError::new($message, $line, $column))
	};
	(Some; $($slices:expr),*; $line:expr, $column:expr) => {
		return Some(Err($crate::json_error::JSONError::new(&format!($($slices),*), $line, $column)))
	};
	($($slices:expr),*; $line:expr, $column:expr) => {
		return Err($crate::json_error::JSONError::new(&format!($($slices),*), $line, $column))
	}
}
