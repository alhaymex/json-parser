#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    StringLiteral(String),
    Number(f64),
    Boolean(bool),
    Null,
    EOF,
}

pub struct Parser {
    content: String,
    cursor: usize,
}

impl Parser {
    pub fn new(content: String) -> Self {
        Self { content, cursor: 0 }
    }

    fn advance(&mut self) {
        if let Some(c) = self.curr_char() {
            self.cursor += c.len_utf8();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.curr_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn curr_char(&self) -> Option<char> {
        self.content[self.cursor..].chars().next()
    }

    fn consume_string(&mut self) -> String {
        let mut string = String::new();
        while let Some(c) = self.curr_char() {
            if c == '"' || c == ','{
                break;
            }
            string.push(c);
            self.advance();
        }
        string
    }

    fn consume_number(&mut self) -> f64 {
        let mut string = String::new();
        while let Some(c) = self.curr_char() {
            if c.is_ascii_digit() || c == '.' || c == '-' {
                string.push(c);
                self.advance();
            } else {
                break;
            }
        }

        string.parse::<f64>().unwrap_or(0.0)
    }

    fn consume_boolean(&mut self) -> bool {
        let mut string = String::new();
        while let Some(c) = self.curr_char() {
            if c.is_alphabetic() {
                string.push(c);
                self.advance();
            } else {
                break;
            }
        }

        string == "true"
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = match self.curr_char() {
            Some(ch) => ch,
            None => return Token::EOF,
        };

        match c {
            '{' => {
                self.advance();
                Token::OpenBrace
            }
            '[' => {
                self.advance();
                Token::OpenBracket
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            '}' => {
                self.advance();
                Token::CloseBrace
            }
            ']' => {
                self.advance();
                Token::CloseBracket
            }
            ',' => {
                self.advance();
                Token::Comma
            }

            '"' => {
                self.advance();
                let s = self.consume_string();
                self.advance();
                Token::StringLiteral(s)
            }

            '0'..='9' | '-' => {
                let n = self.consume_number();
                Token::Number(n)
            }

            't' | 'f' => {
                let b = self.consume_boolean();
                Token::Boolean(b)
            }

            'n' => {
                let word = self.consume_string();
                if word == "null" {
                    Token::Null
                } else {
                    panic!("Unexpected keyword: {}", word)
                }
            }

            _ => panic!("Unexpected character: {c}"),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            match token {
                Token::EOF => break,
                _ => tokens.push(token),
            }
        }

        tokens
    }
}
