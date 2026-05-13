use std::collections::HashMap;

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub struct JsonParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl JsonParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { pos: 0, tokens }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn consume(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: Token) {
        let actual = self.consume();
        if actual != &expected {
            panic!(
                "Syntax Error: Expected {:?}, but found {:?}",
                expected, actual
            );
        }
    }

    pub fn parse(&mut self) -> JsonValue {
        self.parse_value()
    }

    fn parse_value(&mut self) -> JsonValue {
        match self.peek() {
            Token::OpenBrace => self.parse_object(),
            Token::OpenBracket => self.parse_array(),
            Token::Boolean(b) => {
                let b = *b;
                self.consume();
                JsonValue::Boolean(b)
            }
            Token::Number(n) => {
                let n = *n;
                self.consume();
                JsonValue::Number(n)
            }
            Token::Null => {
                self.consume();
                JsonValue::Null
            }
            Token::String(_) => {
                if let Token::String(s) = self.consume() {
                    let s = s.clone();
                    JsonValue::String(s)
                } else {
                    unreachable!()
                }
            }

            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }

    fn parse_object(&mut self) -> JsonValue {
        self.consume();
        let mut map = HashMap::new();

        while *self.peek() != Token::CloseBrace {
            // get the key
            let key = if let Token::String(k) = self.consume() {
                k.clone()
            } else {
                panic!("Expected string key in object");
            };

            // skip the :
            self.expect(Token::Colon);

            let value = self.parse_value();
            map.insert(key, value);

            if *self.peek() == Token::Comma {
                self.consume();
            }
        }
        self.consume();
        JsonValue::Object(map)
    }

    fn parse_array(&mut self) -> JsonValue {
        self.consume();
        let mut vec = Vec::new();

        while *self.peek() != Token::CloseBracket {
            vec.push(self.parse_value());

            if *self.peek() == Token::Comma {
                self.consume();
            }
        }

        self.consume();
        JsonValue::Array(vec)
    }
}
