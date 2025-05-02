use crate::lexer::{Token, Lexer};
use crate::error::CompileError;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
    Variable(String),
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Result<Self, CompileError> {
        let mut parser = Parser {
            lexer,
            current_token: None,
        };
        parser.advance()?;
        Ok(parser)
    }

    fn advance(&mut self) -> Result<(), CompileError> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    pub fn parse_expr(&mut self) -> Result<Expr, CompileError> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expr, CompileError> {
        let mut expr = self.parse_mul_div()?;

        while let Some(token) = &self.current_token {
            match token {
                Token::Add | Token::Sub => {
                    let op = token.clone();
                    self.advance()?;
                    let right = self.parse_mul_div()?;
                    expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, CompileError> {
        let mut expr = self.parse_primary()?;

        while let Some(token) = &self.current_token {
            match token {
                Token::Mul | Token::Div => {
                    let op = token.clone();
                    self.advance()?;
                    let right = self.parse_primary()?;
                    expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, CompileError> {
        match self.current_token.take() {
            Some(Token::Num(n)) => {
                self.advance()?;
                Ok(Expr::Number(n))
            }
            Some(Token::Id(name)) => {
                self.advance()?;
                Ok(Expr::Variable(name))
            }
            Some(t) => Err(CompileError::Parser {
                message: format!("Unexpected token: {:?}", t),
                line: self.lexer.line(),
            }),
            None => Err(CompileError::Parser {
                message: "Unexpected end of input".into(),
                line: self.lexer.line(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::error::CompileError;

    #[test]
    fn test_simple_expression_parse() {
        let lexer = Lexer::new("1 + 2 * 3");
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_expr().unwrap();

        match ast {
            Expr::BinaryOp(_, Token::Add, _) => {}
            _ => panic!("Expected top-level Add"),
        }
    }

    #[test]
    fn test_nested_expression_order() {
        let lexer = Lexer::new("2 * 3 + 4");
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_expr().unwrap();

        match ast {
            Expr::BinaryOp(left, Token::Add, right) => {
                match *left {
                    Expr::BinaryOp(_, Token::Mul, _) => {}
                    _ => panic!("Expected left subtree to be Mul"),
                }
                match *right {
                    Expr::Number(4) => {}
                    _ => panic!("Expected right to be 4"),
                }
            }
            _ => panic!("Unexpected AST structure"),
        }
    }

    #[test]
    fn test_invalid_token_error() {
        let lexer = Lexer::new("@");
        let parser = Parser::new(lexer);
        assert!(parser.is_err());

        match parser {
            Err(CompileError::Lexer { message, line }) => {
                assert!(message.contains("Unknown token"));
                assert_eq!(line, 1);
            }
            other => panic!("Expected lexer error, got {:?}", other),
        }
    }
}
