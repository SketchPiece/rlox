use super::{
    error::report,
    expr::{Expr, Value},
    tokens::{Token, TokenType},
};

use TokenType as TT;
use Value as V;

#[derive(Debug)]
pub enum ParseError {
    ExpectExpression,
    ConsumeUntilTokenNotFound,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.consume_matches(&[TT::BangEqual, TT::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.consume_matches(&[TT::Greater, TT::GreaterEqual, TT::Less, TT::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.consume_matches(&[TT::Minus, TT::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.consume_matches(&[TT::Slash, TT::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.consume_matches(&[TT::Bang, TT::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.consume_matches(&[TT::False]) {
            return Ok(Expr::Literal(V::Bool(false)));
        }
        if self.consume_matches(&[TT::True]) {
            return Ok(Expr::Literal(V::Bool(true)));
        }
        if self.consume_matches(&[TT::Nil]) {
            return Ok(Expr::Literal(V::Nil));
        }

        if self.consume_matches(&[TT::Number(0.0), TT::String("".into())]) {
            let prev_token = self.previous();
            return match prev_token.token_type {
                TT::Number(num) => Ok(Expr::Literal(V::Number(num))),
                TT::String(str) => Ok(Expr::Literal(V::String(str))),
                _ => panic!("The primary neither string nor number despite enum match"),
            };
        }

        if self.consume_matches(&[TT::LeftParen]) {
            let expr = self.expression()?;
            self.consume_expected(TT::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        self.report_error("Expect expression.");

        Err(ParseError::ExpectExpression)
    }

    fn consume_matches(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.consume();
                return true;
            }
        }

        false
    }

    fn consume_expected(
        &mut self,
        expected_type: TokenType,
        message: &str,
    ) -> Result<Token, ParseError> {
        if self.check(&expected_type) {
            Ok(self.consume())
        } else {
            self.report_error(message);
            Err(ParseError::ConsumeUntilTokenNotFound)
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type.variant_eq(token_type)
    }

    fn consume(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::EndOfFile)
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .expect("Peeked out of bounds token")
            .clone()
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .expect("Previous token is out of bounds")
            .clone()
    }

    fn report_error(&self, message: &str) {
        let token = self.peek();
        if token.token_type.variant_eq(&TT::EndOfFile) {
            report(token.line, "at end", message);
        } else {
            report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    // fn synchronize(&mut self) {
    //     self.consume();
    //     while !self.is_at_end() {
    //         if self.previous().token_type.variant_eq(&TT::Semicolon) {
    //             return;
    //         }

    //         match self.peek().token_type {
    //             TT::Class
    //             | TT::Fun
    //             | TT::Var
    //             | TT::For
    //             | TT::If
    //             | TT::While
    //             | TT::Print
    //             | TT::Return => {
    //                 return;
    //             }
    //             _ => {
    //                 self.consume();
    //             }
    //         }
    //     }
    // }
}
