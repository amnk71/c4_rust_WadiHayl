use crate::lexer::{Token, Lexer};

pub enum Expr {
    Number(i64),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
    Variable(String),
    // ... (other expressions)
}

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser { lexer, current_token: None };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Expr {
        let mut expr = self.parse_mul_div();

        while let Some(token) = &self.current_token {
            match token {
                Token::Add | Token::Sub => {
                    let op = token.clone();
                    self.advance();
                    let right = self.parse_mul_div();
                    expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        expr
    }

    fn parse_mul_div(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        while let Some(token) = &self.current_token {
            match token {
                Token::Mul | Token::Div => {
                    let op = token.clone();
                    self.advance();
                    let right = self.parse_primary();
                    expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current_token.take() {
            Some(Token::Num(n)) => {
                self.advance();
                Expr::Number(n)
            }
            Some(Token::Id(name)) => {
                self.advance();
                Expr::Variable(name)
            }
            _ => panic!("Unexpected token"),
        }
    }
}
