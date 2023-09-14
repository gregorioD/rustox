use std::fmt::Display;

use crate::{token::Token, token_type::TokenType};
#[derive(Debug)]
pub struct Unary {
    left: Token,
    expression: Box<Expression>,
}

impl Unary {
    pub fn new(operator: Token, expression: Expression) -> Self {
        match &operator.lexeme()[..] {
            "-" => {
                return Unary {
                    left: operator,
                    expression: Box::new(expression),
                }
            }
            "!" => {
                return Unary {
                    left: operator,
                    expression: Box::new(expression),
                }
            }
            _ => {
                eprintln!("ERROR: invalid left operator for unary expression.");
                panic!();
            }
        }
    }
}
#[derive(Debug)]
pub struct Binary {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>,
}

impl Binary {
    pub fn new(left: Expression, operator: Operator, right: Expression) -> Self {
        Binary {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct Grouping {
    left_brace: char,
    expression: Box<Expression>,
    rigth_brace: char,
}

impl Grouping {
    pub fn new(expression: Expression) -> Self {
        Grouping {
            left_brace: '(',
            expression: Box::new(expression),
            rigth_brace: ')',
        }
    }
}

#[derive(Debug)]
pub struct Operator {
    token: Token,
}

impl Operator {
    pub fn new(operator: Token) -> Self {
        match &operator.lexeme()[..] {
            "==" => return Operator { token: operator },
            "!=" => return Operator { token: operator },
            "<" => return Operator { token: operator },
            "<=" => return Operator { token: operator },
            ">" => return Operator { token: operator },
            ">=" => return Operator { token: operator },
            "+" => return Operator { token: operator },
            "-" => return Operator { token: operator },
            "*" => return Operator { token: operator },
            "/" => return Operator { token: operator },
            _ => {
                eprintln!("ERROR: invalid operator");
                panic!();
            }
        }
    }
}

#[derive(Debug)]
pub struct LiteralExp {
    literal_expression: Token,
}

impl LiteralExp {
    pub fn new(token: Token) -> Self {
        match token.token_type() {
            TokenType::NUMBER => {
                return LiteralExp {
                    literal_expression: token,
                }
            }
            TokenType::STRING => {
                return LiteralExp {
                    literal_expression: token,
                }
            }
            TokenType::TRUE => {
                return LiteralExp {
                    literal_expression: token,
                }
            }
            TokenType::FALSE => {
                return LiteralExp {
                    literal_expression: token,
                }
            }
            TokenType::NIL => {
                return LiteralExp {
                    literal_expression: token,
                }
            }
            _ => {
                eprintln!("ERROR invalid token type for a literal");
                panic!();
            }
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    LiteralExp(LiteralExp),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Operator(Operator),
}
