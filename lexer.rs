//! This module implements the lexical analyzer (lexer) for the compiler.
//! It converts raw source code into a stream of tokens for the parser.

use crate::error::CompileError;

/// Represents the different kinds of tokens that can be identified by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Integer literal (e.g., `42`)
    Num(i64),
    /// Function keyword or type
    Fun,
    /// System call or built-in function
    Sys,
    /// Identifier (e.g., variable or function names)
    Id(String),
    /// `char` keyword
    Char,
    /// `else` keyword
    Else,
    /// `if` keyword
    If,
    /// `int` keyword
    Int,
    /// `return` keyword
    Return,
    /// Assignment operator (`=`)
    Assign,
    /// Addition operator (`+`)
    Add,
    /// Subtraction operator (`-`)
    Sub,
    /// Multiplication operator (`*`)
    Mul,
    /// Division operator (`/`)
    Div,
    // Additional tokens can be added here
}

/// Lexical analyzer that turns source code into a sequence of `Token`s.
#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    /// Creates a new `Lexer` from a source string.

    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    /// Retrieves the next token from the source code.

    pub fn next_token(&mut self) -> Result<Option<Token>, CompileError> {
        while self.pos < self.source.len() {
            match self.source[self.pos] {
                ' ' | '\t' => {
                    self.pos += 1;
                }
                '\n' => {
                    self.line += 1;
                    self.pos += 1;
                }
                '0'..='9' => return Ok(Some(self.read_number())),
                'a'..='z' | 'A'..='Z' | '_' => return Ok(Some(self.read_identifier())),
                '+' => {
                    self.pos += 1;
                    return Ok(Some(Token::Add));
                }
                '-' => {
                    self.pos += 1;
                    return Ok(Some(Token::Sub));
                }
                '*' => {
                    self.pos += 1;
                    return Ok(Some(Token::Mul));
                }
                '/' => {
                    self.pos += 1;
                    return Ok(Some(Token::Div));
                }
                ch => {
                    let err_char = ch;
                    self.pos += 1;
                    return Err(CompileError::Lexer {
                        message: format!("Unknown token '{}'", err_char),
                        line: self.line,
                    });
                }
            }
        }
        Ok(None)
    }

    /// Reads a numeric literal and returns it as a `Token::Num`.
    fn read_number(&mut self) -> Token {
        let mut num = 0;
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_digit() {
            num = num * 10 + self.source[self.pos].to_digit(10).unwrap() as i64;
            self.pos += 1;
        }
        Token::Num(num)
    }

    /// Reads an identifier or keyword and returns the appropriate `Token`.
    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.source.len()
            && (self.source[self.pos].is_ascii_alphanumeric() || self.source[self.pos] == '_')
        {
            self.pos += 1;
        }
        let name: String = self.source[start..self.pos].iter().collect();

        match name.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Id(name),
        }
    }

    /// Returns the current line number in the source code.
    pub fn line(&self) -> usize {
        self.line
    }
}
