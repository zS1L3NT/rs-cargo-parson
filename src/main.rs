use std::fs;

use lexer::Lexer;

mod json;
mod lexer;
mod token;

fn main() {
    let json = fs::read_to_string("files/valid.json").expect("Failed to read file");

    let lexer = Lexer::new(json);
	for token in lexer.get_tokens() {
		println!("{:?}", token);
	}
}
