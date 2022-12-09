# Parson

![License](https://img.shields.io/github/license/zS1L3NT/rs-parson?style=for-the-badge) ![Languages](https://img.shields.io/github/languages/count/zS1L3NT/rs-parson?style=for-the-badge) ![Top Language](https://img.shields.io/github/languages/top/zS1L3NT/rs-parson?style=for-the-badge) ![Commit Activity](https://img.shields.io/github/commit-activity/y/zS1L3NT/rs-parson?style=for-the-badge) ![Last commit](https://img.shields.io/github/last-commit/zS1L3NT/rs-parson?style=for-the-badge)

Parson is a Rust crate that parses a JSON string so that the JSON data can be used in another Rust project

## Motivation

My interest in Rust is growing rapidly due to my boredom of TypeScript and Web Development. Making a Parson in Rust can possibly teach me a lot more about Rust than I currently know. Learning how to make a parser will also teach me about the steps to make a Lexer and a Parser. This is a stepping stone to making my own JavaScript interpreter, my Rust dream project.

## Features

-   JSON Parsing into Rust types
    -   A JSON String parses into a Rust owned string
    -   A JSON Number parses into a Rust f64
    -   A JSON Boolean parses into a Rust bool
    -   A JSON Null is not parsable into Rust since there is no Null value
-   Error Messages
    -   Parser will let you know on what line and column the parsing error occured on

## Installation

Add this to your Rust project's `Cargo.toml` file

```toml
[dependencies]
...
parson = "<version number>"
```

## Usage

Since the [`JSONValue`](#The-JSONValue-struct) struct implements `FromStr`, you can parse it from a string directly into a [`JSONValue`](#The-JSONValue-struct).
If an error occurs in the parsing, you will get back a [`JSONError`](#The-JSONError-struct)

```rs
use parson::JSONValue;

fn main() {
    let json = r#"{ "key": "value" }"#;
    let json_value = json.parse::<JSONValue>().expect("JSON parse failed");
	println!("{}", json_value);
}
```

### The `JSONValue` struct

The `JSONValue` struct holds data contianing either of the following

-   [`JSONString`](#The-JSONString-struct)
-   [`JSONNumber`](#The-JSONNumber-struct)
-   [`JSONBoolean`](#The-JSONBoolean-struct)
-   [`JSONNull`](#The-JSONNull-struct)
-   [`JSONArray`](#The-JSONArray-struct)
-   [`JSONObject`](#The-JSONObject-struct)

`JSONValue` has methods to get the value of each type:

-   `get_string(): Result<String, `[`JSONError`](#The-JSONError-struct)`>`
-   `get_number(): Result<f64, `[`JSONError`](#The-JSONError-struct)`>`
-   `get_boolean(): Result<bool, `[`JSONError`](#The-JSONError-struct)`>`
-   `get_null(): Result<&`[`JSONNull`](#The-JSONNull-struct)`, `[`JSONError`](#The-JSONError-struct)`>`
-   `get_array(): Result<&`[`JSONArray`](#The-JSONArray-struct)`, `[`JSONError`](#The-JSONError-struct)`>`
-   `get_object(): Result<&`[`JSONObject`](#The-JSONObject-struct)`, `[`JSONError`](#The-JSONError-struct)`>`

`JSONValue` also has methods to check if the value is of each type

-   `is_string(): bool`
-   `is_number(): bool`
-   `is_boolean(): bool`
-   `is_null(): bool`
-   `is_array(): bool`
-   `is_object(): bool`

`JSONValue` has a method which returns an enum of the type of value it holds

```rs
let json_value = /* result of parsing a string */

match json_value.get_type() {
	JSONType::String(json_string) => println!("{}", json_string.get_string()),
	JSONType::Number(json_number) => println!("{}", json_number.get_number()),
	JSONType::Boolean(json_boolean) => println!("{}", json_boolean.get_boolean()),
	JSONType::Null(json_null) => println!("Null"),
	JSONType::Array(json_array) => println!("{} items", json_array.len()),
	JSONType::Object(json_object) => println!("{} pairs", json_object.len()),
}
```

### The `JSONString` struct

The `JSONString` struct holds data about a string in your input json as a `String`.
The `JSONString` implementation has 1 method to get the data within it.

-   `get_string(): String`

### The `JSONNumber` struct

The `JSONNumber` struct holds data about a number in your input json as an `f64`.
The `JSONNumber` implementation has 1 method to get the data within it.

-   `get_number(): f64`

### The `JSONBoolean` struct

The `JSONBoolean` struct holds data about a boolean in your input json as a `bool`.
The `JSONBoolean` implementation has 1 method to get the data within it.

-   `get_boolean(): bool`

### The `JSONNull` struct

The `JSONNull` struct is a representation of a null value in your input json.
The `JSONNull` implementation has no methods to get the data within it since Rust has no way to represent Null values.

### The `JSONArray` struct

The `JSONArray` struct holds data about an array in your input json as a `Vec<`[`JSONValue`](#The-JSONValue-struct)`>`.
The `JSONArray` implementation has 3 methods to get the data within it.

-   `to_vec(): Vec<`[`JSONValue`](#The-JSONValue-struct)`>`
-   `len(): usize`
-   `get(index: usize): `[`JSONValue`](#The-JSONValue-struct)

### The `JSONObject` struct

The `JSONObject` struct holds data about an object in your input json as a `IndexMap<String, `[`JSONValue`](#The-JSONValue-struct)`>`, which is a HashMap that maintains the order data was added to it.
The `JSONObject` implementation has 3 methods to get the data within it.

-   `to_hashmap(): HashMap<String, `[`JSONValue`](#The-JSONValue-struct)`>`
-   `len(): usize`
-   `get(key: &str): `[`JSONValue`](#The-JSONValue-struct)

### The `JSONError` struct

The `JSONError` struct holds the message of an error that happened when parsing your input json.

`JSONError` has 1 method to get the error message within it.

-   `get_message(): String`

## Credits

I also learnt how to make a basic JSON Parser from [here](https://notes.eatonphil.com/writing-a-simple-parson.html). I understood and improvised the way the author wrote the Lexer and Parsers to make Parson more bug free.

## Built with

-   Rust
    -   [![indexmap](https://img.shields.io/badge/indexmap-%5E1.8.0-yellow?style=flat-square)](https://crates.io/crates/indexmap/1.8.0)
