use std::rc::Rc;

use super::{
    ast::{Expr, Stmt, Value},
    reporter::ErrorReporter,
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
    reporter: Option<Rc<dyn ErrorReporter>>,
}

type Result<T> = std::result::Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            reporter: None,
        }
    }

    pub fn attach_reporter<R>(mut self, reporter: Rc<R>) -> Self
    where
        R: ErrorReporter + 'static,
    {
        self.reporter = Some(reporter);
        self
    }

    fn report_error(&self, token: &Token, message: &str) {
        if let Some(reporter) = self.reporter.as_ref() {
            if token.token_type.variant_eq(&TT::EndOfFile) {
                reporter.report(token.line, " at end", message);
            } else {
                reporter.report(token.line, &format!(" at '{}'", token.lexeme), message);
            }
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                // panic mode!
                // recover from invalid statement and continue parsing
                Err(_) => self.synchronize(),
            }
        }

        statements
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.consume_matches(&[TT::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume_expected(TT::Identifier, "Expect variable name.")?;

        let mut initializer = None;

        if self.consume_matches(&[TT::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume_expected(TT::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.consume_matches(&[TT::Print]) {
            self.print_statement()
        } else if self.consume_matches(&[TT::LeftBrace]) {
            Ok(Stmt::Block(Box::new(self.block()?)))
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume_expected(TT::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume_expected(TT::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.check(&TT::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume_expected(TT::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.equality()?;

        if self.consume_matches(&[TT::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }

            self.report_error(&equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
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

    fn comparison(&mut self) -> Result<Expr> {
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

    fn term(&mut self) -> Result<Expr> {
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

    fn factor(&mut self) -> Result<Expr> {
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

    fn unary(&mut self) -> Result<Expr> {
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

    fn primary(&mut self) -> Result<Expr> {
        if self.consume_matches(&[TT::False]) {
            Ok(Expr::Literal(V::Bool(false)))
        } else if self.consume_matches(&[TT::True]) {
            Ok(Expr::Literal(V::Bool(true)))
        } else if self.consume_matches(&[TT::Nil]) {
            Ok(Expr::Literal(V::Nil))
        } else if self.consume_matches(&[TT::Number(0.0), TT::String("".into())]) {
            let prev_token = self.previous();
            match prev_token.token_type {
                TT::Number(num) => Ok(Expr::Literal(V::Number(num))),
                TT::String(str) => Ok(Expr::Literal(V::String(str))),
                _ => panic!("the primary value neither string nor number despite enum match"),
            }
        } else if self.consume_matches(&[TT::Identifier]) {
            Ok(Expr::Variable(self.previous()))
        } else if self.consume_matches(&[TT::LeftParen]) {
            let expr = self.expression()?;
            self.consume_expected(TT::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        } else {
            self.report_error(self.peek(), "Expect expression.");
            Err(ParseError::ExpectExpression)
        }
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

    fn consume_expected(&mut self, expected_type: TokenType, message: &str) -> Result<Token> {
        if self.check(&expected_type) {
            Ok(self.consume())
        } else {
            self.report_error(self.peek(), message);
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

        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::EndOfFile)
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("current must be within tokens bounds")
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .expect("previous token must be used after it was consumed at least once")
            .clone()
    }

    fn synchronize(&mut self) {
        self.consume();
        while !self.is_at_end() {
            if self.previous().token_type.variant_eq(&TT::Semicolon) {
                return;
            }

            match self.peek().token_type {
                TT::Class
                | TT::Fun
                | TT::Var
                | TT::For
                | TT::If
                | TT::While
                | TT::Print
                | TT::Return => {
                    return;
                }
                _ => {
                    self.consume();
                }
            }
        }
    }
}
