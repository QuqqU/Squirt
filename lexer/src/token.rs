use std::fmt;

#[derive(Clone, PartialEq)]
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
                write!(f, "{}({})", self.token_type, self.literal)
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
                    "Token{{ {}({}) : ({}, {}) }}",
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match *self {
            TokenType::Poison => "__poison__",
            TokenType::Eof => "__EOF__",
            TokenType::Ident => "__ident__",
            TokenType::Int => "__int__",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Bang => "!",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Eq => "==",
            TokenType::Neq => "!=",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Lparen => "(",
            TokenType::Rparen => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",
            TokenType::Let => "let",
            TokenType::Func => "func",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Return => "return",
        };
        write!(f, "{}", token)
    }
}

pub fn look_up_ident(s: &str) -> TokenType {
    match s {
        "let" => TokenType::Let,
        "fn" => TokenType::Func,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}
