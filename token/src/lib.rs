mod literal;
mod util;

pub use literal::*;
pub use util::look_up_ident;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Poison,
    Eof,
    Ident(String),
    Int(i64),
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Lt,
    Gt,
    Eq,
    Neq,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Let,
    Func,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    pub fn name(&self) -> Result<&str, &'static str> {
        match self {
            Token::Ident(ident) => Ok(ident),
            _ => Err("TKN0100 : Token is not a Identifier"),
        }
    }

    pub fn value(&self) -> Result<i64, &'static str> {
        match self {
            Token::Int(i) => Ok(*i),
            _ => Err("TKN0101 : Token is not a Int"),
        }
    }
}
