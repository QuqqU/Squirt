use std::any::Any;
use token::TokenType;

pub struct Identifier {
    pub token: TokenType, // token::IDENT
    pub value: String,
}
pub trait Statement {
    fn as_any(&self) -> &dyn Any;
}

pub struct Expression {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
impl Program {
    pub fn empty(&self) -> bool {
        self.statements.is_empty()
    }
}

pub struct UndefinedStatement {}
impl Statement for UndefinedStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LetStatement {
    // let ident = value
    pub token: TokenType, // token::LET
    pub name:  Identifier,
    pub value: Expression,
}
impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


