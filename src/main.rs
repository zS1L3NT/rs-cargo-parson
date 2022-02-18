use std::fs;

use json::{object::JSONObject, JSONValue};
use lexer::Lexer;
use parser::Parser;

mod json;
mod lexer;
mod parser;
mod token;

fn main() {
    let json = fs::read_to_string("files/valid.json").expect("Failed to read file");

    let lexer = Lexer::new(json);
    let parser = Parser::new(lexer.get_tokens());
    let json = parser.parse().downcast::<JSONObject>().unwrap();
	println!("{}", json.to_string());
}
