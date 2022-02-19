//! # Parse JSON
//!
//! `parse-json` is a Rust library for parsing JSON strings into Rust types.
//!
//! In this crate, a
//! - JSON String is parsed into a Rust owned string
//! - JSON Number is parsed into a Rust f64
//! - JSON Boolean is parsed into a Rust bool
//! - JSON Null is not parsable into Rust since there is no Null value

pub use json_array::JSONArray;
pub use json_boolean::JSONBoolean;
pub use json_null::JSONNull;
pub use json_number::JSONNumber;
pub use json_object::JSONObject;
pub use json_string::JSONString;
pub use json_value::JSONType;
pub use json_value::JSONValue;
use token::Token;

mod json_array;
mod json_boolean;
mod json_null;
mod json_number;
mod json_object;
mod json_string;
mod json_value;
mod lexer;
mod parser;
mod token;
