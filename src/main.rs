use std::fs;

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
    let value = parser.parse();
    let json = value.get_object();
    println!("{}", json.to_string());
}
