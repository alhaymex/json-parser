use std::fs;

use crate::parser::Parser;

mod parser;

const FILE_PATH: &str = "file.json";

fn main() {
    let content = fs::read_to_string(FILE_PATH).expect("Could not read file");
    let mut parser = Parser::new(content);
    let tokens = parser.parse();

    println!("{:#?}", tokens);
}
