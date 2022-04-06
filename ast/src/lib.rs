use std::any::Any;
use token::TokenType;

mod util;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: TokenType, // token::IDENT
    pub value: String,
}
// pub type BlockStatement = Vec<Statement>;

pub trait Node {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, PartialEq, Clone)]
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
    If {
        token:       TokenType,
        condition:   Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Vec<Statement>,
    },
    FunctionLiteral {
        token:      TokenType,
        parameters: Vec<Identifier>,
        body:       Vec<Statement>,
    },
    FunctionCall {
        token: TokenType,       // token::LPAREN
        func:  Box<Expression>, // functionliteral or ident
        args:  Vec<Expression>,
    },
}
impl Node for Expression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
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
impl Node for Statement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
impl Node for Program {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
