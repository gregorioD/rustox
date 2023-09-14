use crate::{
    error::Error,
    expression::{Binary, Expression, Grouping, LiteralExp, Operator, Unary},
    token::Token,
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type() == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type() {
                TokenType::CLASS => return,
                TokenType::FUN => return,
                TokenType::VAR => return,
                TokenType::FOR => return,
                TokenType::IF => return,
                TokenType::WHILE => return,
                TokenType::PRINT => return,
                TokenType::RETURN => return,
                _ => (),
            }

            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Expression, Error> {
        match self.expression() {
            Ok(exp) => Ok(exp),
            Err(e) => Err(e),
        }
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, Error> {
        match self.comparison() {
            Ok(mut expr) => {
                while self.match_token(&[TokenType::BANG, TokenType::BANG_EQUAL]) {
                    let operator = Operator::new(self.previous());
                    let right = self.comparison()?;
                    expr = Expression::Binary(Binary::new(expr, operator, right));
                }
                Ok(expr)
            }
            Err(e) => Err(e),
        }
    }

    fn comparison(&mut self) -> Result<Expression, Error> {
        match self.term() {
            Ok(mut expr) => {
                while self.match_token(&[
                    TokenType::GREATER,
                    TokenType::GREATER_EQUAL,
                    TokenType::LESS,
                    TokenType::LESS_EQUAL,
                ]) {
                    let operator = Operator::new(self.previous());
                    let right = self.term()?;
                    expr = Expression::Binary(Binary::new(expr, operator, right));
                }
                Ok(expr)
            }
            Err(e) => Err(e),
        }
    }

    fn term(&mut self) -> Result<Expression, Error> {
        match self.factor() {
            Ok(mut expr) => {
                while self.match_token(&[TokenType::MINUS, TokenType::PLUS]) {
                    let operator = Operator::new(self.previous());
                    let right = self.factor()?;
                    expr = Expression::Binary(Binary::new(expr, operator, right));
                }
                Ok(expr)
            }
            Err(e) => Err(e),
        }
    }

    fn factor(&mut self) -> Result<Expression, Error> {
        match self.unary() {
            Ok(mut expr) => {
                while self.match_token(&[TokenType::SLASH, TokenType::STAR]) {
                    let operator = Operator::new(self.previous());
                    let right = self.unary()?;
                    expr = Expression::Binary(Binary::new(expr, operator, right));
                }
                Ok(expr)
            }
            Err(e) => Err(e),
        }
    }

    fn unary(&mut self) -> Result<Expression, Error> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            match self.unary() {
                Ok(right) => return Ok(Expression::Unary(Unary::new(operator, right))),
                Err(e) => return Err(e),
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        if self.match_token(&[TokenType::FALSE]) {
            return Ok(Expression::LiteralExp(LiteralExp::new(self.previous())));
        }
        if self.match_token(&[TokenType::TRUE]) {
            return Ok(Expression::LiteralExp(LiteralExp::new(self.previous())));
        }
        if self.match_token(&[TokenType::NIL]) {
            return Ok(Expression::LiteralExp(LiteralExp::new(self.previous())));
        }

        if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expression::LiteralExp(LiteralExp::new(self.previous())));
        }

        if self.match_token(&[TokenType::LEFT_PAREN]) {
            let expression = self.expression()?;
            match self.consume(
                TokenType::RIGHT_PAREN,
                "Expect ')' after expression.".to_string(),
            ) {
                Ok(_) => return Ok(Expression::Grouping(Grouping::new(expression))),
                Err(e) => return Err(e),
            }
        } else {
            return Err(Error::new(
                self.peek().line(),
                "No primary expression matched.".to_string(),
            ));
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        let mut result = false;

        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                result = true;
                break;
            }
        }

        result
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            return self.peek().token_type() == token_type;
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type() == TokenType::EOF
    }

    fn peek(&self) -> Token {
        match self.tokens.get(self.current) {
            Some(token) => return token.clone(),
            None => panic!(), // TODO: ERROR HANDLING
        }
    }

    fn previous(&self) -> Token {
        if self.current > 0 {
            match self.tokens.get(self.current - 1) {
                Some(token) => return token.clone(),
                None => panic!(), // TODO
            }
        } else {
            eprintln!("Current position: 0");
            panic!(); // TODO
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        } else {
            return Err(self.scan_error(message));
        }
    }

    fn scan_error(&self, message: String) -> Error {
        let line = self.peek().line();
        Error::new(line, message)
    }
}
