use std::fs;

mod lexer;
mod parser;

use crate::lexer::Lexer;
use crate::parser::{JsonParser, JsonValue};

const FILE_PATH: &str = "file.json";

fn main() {
    let content = fs::read_to_string(FILE_PATH).expect("Could not read file");
    let mut lexer = Lexer::new(content);
    let tokens = lexer.parse();

    println!("{:#?}", tokens);

    let mut parser = JsonParser::new(tokens);
    let data: JsonValue = parser.parse();

    println!("{:?}", data);
}
