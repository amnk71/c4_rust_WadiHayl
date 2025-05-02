#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(i64),
    Fun,
    Sys,
    Id(String),
    Char,
    Else,
    If,
    Int,
    Return,
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    // ... (other tokens)
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while self.pos < self.source.len() {
            match self.source[self.pos] {
                ' ' | '\t' => { self.pos += 1; }
                '\n' => { self.line += 1; self.pos += 1; }
                '0'..='9' => return Some(self.read_number()),
                'a'..='z' | 'A'..='Z' | '_' => return Some(self.read_identifier()),
                '+' => { self.pos += 1; return Some(Token::Add); }
                '-' => { self.pos += 1; return Some(Token::Sub); }
                '*' => { self.pos += 1; return Some(Token::Mul); } // ✅
                '/' => { self.pos += 1; return Some(Token::Div); } // ✅
                // ... (handle other operators)
                _ => panic!("Unknown token at line {}", self.line),
            }
        }
        None
    }

    fn read_number(&mut self) -> Token {
        let mut num = 0;
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_digit() {
            num = num * 10 + self.source[self.pos].to_digit(10).unwrap() as i64;
            self.pos += 1;
        }
        Token::Num(num)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.source.len() && (self.source[self.pos].is_ascii_alphanumeric() || self.source[self.pos] == '_') {
            self.pos += 1;
        }
        let name: String = self.source[start..self.pos].iter().collect();

        match name.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            // ... (other keywords)
            _ => Token::Id(name),
        }
    }
}
