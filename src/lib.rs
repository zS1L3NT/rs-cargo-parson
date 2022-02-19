pub use json::array::JSONArray;
pub use json::boolean::JSONBoolean;
pub use json::null::JSONNull;
pub use json::number::JSONNumber;
pub use json::object::JSONObject;
pub use json::string::JSONString;
pub use json::value::JSONValue;
pub use json::value::JSONType;
pub use lexer::Lexer;
pub use parser::Parser;
pub use token::Token;

pub mod json;
pub mod lexer;
pub mod parser;
pub mod token;

pub fn parse(json: String) -> JSONValue {
    let lexer = Lexer::new(json);
    let parser = Parser::new(lexer.get_tokens());
    parser.parse()
}
