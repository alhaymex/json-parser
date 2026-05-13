use std::{fs, panic};

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

    println!("type Type = {}", to_type(&data));
}

fn to_type(value: &JsonValue) -> String {
    match value {
        JsonValue::String(_) => String::from("string"),
        JsonValue::Number(_) => String::from("number"),
        JsonValue::Null => String::from("null"),
        JsonValue::Boolean(_) => String::from("boolean"),

        JsonValue::Array(items) => {
            if items.is_empty() {
                String::from("unknown[]")
            } else {
                format!("{}[]", to_type(&items[0]))
            }
        }

        JsonValue::Object(map) => {
            let mut fields = Vec::new();

            for (key, value) in map {
                fields.push(format!("{}: {};", key, to_type(value)));
            }

            format!("{{\n{}\n}}", fields.join("\n"))
        }
    }
}
