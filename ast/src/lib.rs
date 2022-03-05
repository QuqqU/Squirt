use std::fmt::*;

use token::TokenType;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: TokenType, // token::IDENT
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Undefined,
    Ident(Identifier),
    IntegerLiteral {
        token: TokenType,
        value: i64,
    },
    Prefix {
        token:    TokenType,
        operator: String,
        right:    Box<Expression>,
    },
    Infix {
        token:    TokenType,
        left:     Box<Expression>,
        operator: String,
        right:    Box<Expression>,
    },
    Bool {
        token: TokenType,
        value: bool,
    },
}
impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Self::Ident(i) => format!("{}", i.value),
            Self::IntegerLiteral { token: _, value } => format!("{}", value),
            Self::Prefix {
                token: _,
                operator,
                right,
            } => format!("({}{})", operator, right.to_string()),
            Self::Infix {
                token: _,
                left,
                operator,
                right,
            } => format!("({} {} {})", left.to_string(), operator, right.to_string()),
            Self::Bool { token: _, value } => format!("{}", value),
            _ => format!("Undefined"),
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Let {
        token: TokenType, // token::LET
        name:  Identifier,
        value: Expression,
    },
    Return {
        token: TokenType, // token::RETURN
        value: Expression,
    },
    Expr {
        token:      TokenType, //expression 의 첫 토큰
        expression: Expression,
    },
}
impl Statement {
    pub fn to_string(&self) -> String {
        match self {
            Self::Let {
                token: _,
                name,
                value,
            } => format!("let {} = {};", name.value, value.to_string()),
            Self::Return { token: _, value } => format!("return {};", value.to_string()),
            Self::Expr {
                token: _,
                expression,
            } => format!("{}", expression.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn empty(&self) -> bool {
        self.statements.is_empty()
    }
}
