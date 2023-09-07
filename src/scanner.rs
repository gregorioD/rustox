use crate::{
    error::Error,
    token::{Literal, Token},
    token_type::TokenType,
};

use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub error: Option<Error>,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let keywords: HashMap<String, TokenType> = HashMap::from([
            ("and".to_string(), TokenType::AND),
            ("class".to_string(), TokenType::CLASS),
            ("else".to_string(), TokenType::ELSE),
            ("false".to_string(), TokenType::FALSE),
            ("for".to_string(), TokenType::FOR),
            ("fun".to_string(), TokenType::FUN),
            ("if".to_string(), TokenType::IF),
            ("nil".to_string(), TokenType::NIL),
            ("or".to_string(), TokenType::OR),
            ("print".to_string(), TokenType::PRINT),
            ("return".to_string(), TokenType::RETURN),
            ("super".to_string(), TokenType::SUPER),
            ("this".to_string(), TokenType::THIS),
            ("true".to_string(), TokenType::TRUE),
            ("var".to_string(), TokenType::VAR),
            ("while".to_string(), TokenType::WHILE),
        ]);
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error: None,
            keywords: keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            String::from(""),
            TokenType::EOF,
            self.line,
            None,
        ));

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        if self.current >= self.source.len() {
            return true;
        }

        false
    }

    fn char_at(&self, position: usize) -> char {
        self.source.as_bytes()[position] as char
    }

    fn substring(&self, start: usize, end: usize) -> String {
        self.source[self.start..self.current].to_string()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.addToken(TokenType::LEFT_PAREN, None),
            ')' => self.addToken(TokenType::RIGHT_PAREN, None),
            '{' => self.addToken(TokenType::LEFT_BRACE, None),
            '}' => self.addToken(TokenType::RIGHT_BRACE, None),
            ',' => self.addToken(TokenType::COMMA, None),
            '.' => self.addToken(TokenType::DOT, None),
            '-' => self.addToken(TokenType::MINUS, None),
            '+' => self.addToken(TokenType::PLUS, None),
            '*' => self.addToken(TokenType::STAR, None),
            ';' => self.addToken(TokenType::SEMICOLON, None),
            '!' => {
                if self.advance_if_match('=') {
                    self.addToken(TokenType::BANG_EQUAL, None);
                } else {
                    self.addToken(TokenType::BANG, None);
                }
            }
            '=' => {
                if self.advance_if_match('=') {
                    self.addToken(TokenType::EQUAL_EQUAL, None);
                } else {
                    self.addToken(TokenType::EQUAL, None);
                }
            }
            '<' => {
                if self.advance_if_match('=') {
                    self.addToken(TokenType::LESS_EQUAL, None);
                } else {
                    self.addToken(TokenType::LESS, None);
                }
            }
            '>' => {
                if self.advance_if_match('=') {
                    self.addToken(TokenType::GREATER_EQUAL, None);
                } else {
                    self.addToken(TokenType::GREATER, None);
                }
            }
            '/' => {
                if self.advance_if_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.addToken(TokenType::SLASH, None);
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            c => {
                if c.is_digit(10) {
                    self.number();
                } else if Self::is_my_alphabetic(c) {
                    self.identifier();
                } else {
                    self.error(self.line, "Unexpected character".to_string());
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.char_at(self.current);
        self.current += 1;
        std::char::from_u32(c as u32).unwrap()
    }

    fn addToken(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.substring(self.start, self.current);
        self.tokens
            .push(Token::new(text, token_type, self.line, literal));
    }

    fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: usize, where_happened: String, message: String) {
        eprintln!("[{line}] Error {where_happened}: {message}");
        self.error = Some(Error::new(line, message));
    }

    fn advance_if_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.char_at(self.current) != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.char_at(self.current);
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        } else {
            return self.char_at(self.current + 1);
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.".to_string());
            return;
        }
        self.advance();
        let value: String = self.substring(self.start + 1, self.current - 1);
        self.addToken(TokenType::STRING, Some(Literal::String(value)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let number: f32 = self.substring(self.start, self.current).parse().unwrap();

        self.addToken(TokenType::NUMBER, Some(Literal::Float(number)));
    }

    fn identifier(&mut self) {
        while Self::is_my_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.substring(self.start, self.current);
        if let Some(token_type) =
            self.keywords()
                .iter()
                .find_map(|(key, val)| if *key == text { Some(*val) } else { None })
        {
            self.addToken(token_type, None)
        } else {
            self.addToken(TokenType::IDENTIFIER, None);
        }
    }

    fn keywords(&self) -> HashMap<String, TokenType> {
        self.keywords.clone()
    }

    fn is_my_alphabetic(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
    }
    fn is_my_alphanumeric(c: char) -> bool {
        c.is_digit(10) || Self::is_my_alphabetic(c)
    }
}
