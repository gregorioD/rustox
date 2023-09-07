use crate::{
    token::{Literal, Token},
    token_type::TokenType,
};

struct Unary {
    left: Token,
    expression: Box<Expression>,
}

impl Unary {
    fn new(operator: Token, expression: Expression) -> Self {
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

struct Binary {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>,
}

struct Grouping {
    left_brace: char,
    expression: Box<Expression>,
    rigth_brace: char,
}

impl Grouping {
    fn new(expression: Expression) -> Self {
        Grouping {
            left_brace: '(',
            expression: Box::new(expression),
            rigth_brace: ')',
        }
    }
}

struct Operator {
    token: Token,
}

impl Operator {
    fn new(operator: Token) -> Self {
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

struct LiteralExp {
    literal_expression: Token,
}

impl LiteralExp {
    fn new(token: Token) -> Self {
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

enum Expression {
    LiteralExp(LiteralExp),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Operator(Operator),
}
