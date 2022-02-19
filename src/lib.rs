//! # Parse JSON
//!
//! `parse-json` is a Rust library for parsing JSON strings into Rust types.
//!
//! In this crate, a
//! - JSON String is parsed into a Rust owned string
//! - JSON Number is parsed into a Rust f64
//! - JSON Boolean is parsed into a Rust bool
//! - JSON Null is not parsable into Rust since there is no Null value

pub use json::array::JSONArray;
pub use json::boolean::JSONBoolean;
pub use json::null::JSONNull;
pub use json::number::JSONNumber;
pub use json::object::JSONObject;
pub use json::string::JSONString;
pub use json::value::JSONType;
pub use json::value::JSONValue;
use token::Token;
use lexer::Lexer;
use parser::Parser;

pub mod json;
mod lexer;
mod parser;
mod token;

/// Parse JSON in the form of a `&str` into a JSON Value
pub fn parse(json: &str) -> JSONValue {
    let lexer = Lexer::new(json.to_string());
    let parser = Parser::new(lexer.get_tokens());
    parser.parse()
}
