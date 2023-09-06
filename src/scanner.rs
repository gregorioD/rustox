use crate::{
    error::Error,
    lox::Lox,
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Some(e) => return Err(e),
                None => (),
            }
        }

        self.tokens.push(Token::new(
            String::from(""),
            TokenType::EOF,
            self.line,
            None,
        ));

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        if self.current >= self.source.len() {
            return true;
        }

        false
    }

    fn scan_token(&mut self) -> Option<Error> {
        let c: char = self.advance();

        match c {
            '(' => self.addToken(TokenType::LEFT_PAREN, None),
            ')' => self.addToken(TokenType::LEFT_PAREN, None),
            '{' => self.addToken(TokenType::LEFT_BRACE, None),
            '}' => self.addToken(TokenType::RIGHT_BRACE, None),
            ',' => self.addToken(TokenType::COMMA, None),
            '.' => self.addToken(TokenType::DOT, None),
            '-' => self.addToken(TokenType::MINUS, None),
            '+' => self.addToken(TokenType::PLUS, None),
            '*' => self.addToken(TokenType::STAR, None),
            ';' => self.addToken(TokenType::SEMICOLON, None),
            _ => {}
        }

        None
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current as usize];
        self.current += 1;
        std::char::from_u32(c as u32).unwrap()
    }

    fn addToken(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: &str = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(
            String::from(text),
            token_type,
            self.line,
            literal,
        ));
    }
}
