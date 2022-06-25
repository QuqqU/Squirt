mod literal;
mod util;

pub use literal::*;
pub use util::look_up_ident;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Poison,
    Eof,
    Ident,
    Int,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal:    String,
    pub row:        i64,
    pub column:     i64,
}

// impl TokenT {
//     pub fn name(&self) -> Result<&str, &'static str> {
//         match self {
//             Token::Ident(ident) => Ok(ident),
//             _ => Err("TKN0100 : Token is not a Identifier"),
//         }
//     }

//     pub fn value(&self) -> Result<i64, &'static str> {
//         match self {
//             Token::Int(i) => Ok(*i),
//             _ => Err("TKN0101 : Token is not a Int"),
//         }
//     }
// }
