use crate::{
    error::error,
    tokens::{Token, TokenType},
};
use std::{char, collections::HashMap, mem};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn get_keywords() -> HashMap<&'static str, TokenType> {
        HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ])
    }

    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token {
            token_type: TokenType::EndOfFile,
            lexeme: "".to_owned(),
            line: self.line,
        });

        mem::take(&mut self.tokens)
    }

    fn scan_token(&mut self) {
        let consumed_char = self.consume().expect("Failed to consume next character");

        match consumed_char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_token_with_match('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_token_with_match('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.add_token_with_match('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.add_token_with_match('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => {
                if self.next_matches('/') {
                    self.consume_comment();
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '\n' => {
                self.line += 1;
            }
            ' ' | '\r' | '\t' => (),
            '"' => self.consume_string(),
            _ => {
                if consumed_char.is_ascii_digit() {
                    self.consume_number();
                } else if consumed_char.is_alphabetic() {
                    self.consume_identifier();
                } else {
                    error(self.line, "unexpected char (fix error)")
                }
            }
        };
    }

    fn consume_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.consume();
        }

        let text = &self.source[self.start..self.current];
        if let Some(token_type) = Scanner::get_keywords().get(text).cloned() {
            self.add_token(token_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn consume_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.consume();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }

        // consume the closing "
        self.consume();

        let value = self.source[self.start + 1..self.current - 1].to_owned();

        self.add_token(TokenType::String(value));
    }

    fn consume_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.consume();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.consume();

            while self.peek().is_ascii_digit() {
                self.consume();
            }

            let number_literal = self.source[self.start..self.current].to_owned();
            let value: f64 = number_literal
                .parse()
                .expect("Consumed string is not a number");

            self.add_token(TokenType::Number(value));
        }
    }

    fn consume_comment(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.consume();
        }
    }

    fn add_token_with_match(
        &mut self,
        expected: char,
        expected_token: TokenType,
        else_token: TokenType,
    ) {
        if self.next_matches(expected) {
            self.add_token(expected_token)
        } else {
            self.add_token(else_token)
        }
    }

    fn next_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };

        match self.source.chars().nth(self.current) {
            Some(next_char) => {
                if next_char == expected {
                    self.current += 1;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn consume(&mut self) -> Option<char> {
        let consumed_char = self.source.chars().nth(self.current);
        self.current += 1;
        consumed_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_owned();

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            line: self.line,
        })
    }
}
