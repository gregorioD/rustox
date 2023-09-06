use std::fmt::Display;

use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    literal: Option<Literal>,
}

#[derive(Clone)]
pub enum Literal {
    String(String),
    Float(f32),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Float(float) => write!(f, "{}", float),
            Literal::String(string) => write!(f, "{}", string),
        }
    }
}

impl Token {
    pub fn new(
        lexeme: String,
        token_type: TokenType,
        line: usize,
        literal_opt: Option<Literal>,
    ) -> Self {
        if let Some(literal) = literal_opt {
            return Token {
                token_type: token_type,
                lexeme: lexeme,
                line: line,
                literal: Some(literal),
            };
        }

        Token {
            token_type: token_type,
            lexeme: lexeme,
            line: line,
            literal: None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = self.token_type.to_string();
        let str2 = self.lexeme.clone();

        if let Some(literal) = &self.literal {
            write!(f, "{} {} {}", str1, str2, literal.to_string().clone())
        } else {
            write!(f, "{} {}", str1, str2)
        }
    }
}

/*
impl ToString for Token {
    fn to_string(&self) -> String {
        let str1 = self.token_type.to_string();
        let str2 = self.lexeme.clone();

        if let Some(literal) = &self.literal {
            format!("{} {} {}", str1, str2, literal.to_string().clone())
        } else {
            format!("{} {}", str1, str2)
        }
    }
}
*/
