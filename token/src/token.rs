use std::fmt;

use crate::tokentype::TokenType;

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal:    String,
    pub row:        i64,
    pub column:     i64,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str, row: i64, column: i64) -> Self {
        Token {
            token_type,
            literal: literal.to_string(),
            row,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token_type {
            TokenType::Poison | TokenType::Eof | TokenType::Ident | TokenType::Int => {
                write!(f, "{:?}({})", self.token_type, self.literal)
            }
            _ => write!(f, "{}", self.literal),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token_type {
            TokenType::Poison | TokenType::Eof | TokenType::Ident | TokenType::Int => {
                write!(
                    f,
                    "Token{{ {:?}({}) : ({}, {}) }}",
                    self.token_type, self.literal, self.row, self.column
                )
            }
            _ => write!(
                f,
                "Token{{ {} : ({}, {}) }}",
                self.literal, self.row, self.column
            ),
        }
    }
}
