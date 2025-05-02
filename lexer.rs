use crate::error::CompileError;

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
  
}

#[derive(Debug)] 
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

    pub fn line(&self) -> usize {
        self.line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("1 + 2 - 3 * 4 / 5");

        assert_eq!(lexer.next_token().unwrap(), Some(Token::Num(1)));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Add));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Num(2)));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Sub));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Num(3)));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Mul));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Num(4)));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Div));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Num(5)));
        assert_eq!(lexer.next_token().unwrap(), None);
    }

    #[test]
    fn test_identifier_and_keywords() {
        let mut lexer = Lexer::new("if else return varName");

        assert_eq!(lexer.next_token().unwrap(), Some(Token::If));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Else));
        assert_eq!(lexer.next_token().unwrap(), Some(Token::Return));
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::Id("varName".to_string()))
        );
    }

    #[test]
    fn test_invalid_token() {
        let mut lexer = Lexer::new("$");
        let result = lexer.next_token();
        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                CompileError::Lexer { message, line } => {
                    assert_eq!(line, 1);
                    assert!(message.contains("Unknown token"));
                }
                _ => panic!("Expected lexer error"),
            }
        }
    }

    #[test]
    fn test_direct_lexer_rejects_at_symbol() {
        let mut lexer = Lexer::new("@");
        let result = lexer.next_token();
        assert!(result.is_err());

        if let Err(CompileError::Lexer { message, line }) = result {
            assert_eq!(line, 1);
            assert!(message.contains("Unknown token"));
        } else {
            panic!("Expected lexer error for '@'");
        }
    }
}
