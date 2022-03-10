use std::{any::Any, fmt::*};

use token::TokenType;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: TokenType, // token::IDENT
    pub value: String,
}
// pub type BlockStatement = Vec<Statement>;

pub trait Node {
    fn as_any(&self) -> &dyn Any;
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
        token: TokenType, // token::LPAREN
        func:  Box<Expression>,
        args:  Vec<Expression>,
    },
}
impl Node for Expression {
    fn as_any(&self) -> &dyn Any {
        self
    }
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
            Self::If {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                if !alternative.is_empty() {
                    format!(
                        "if({}) {{ {} }} else {{ {} }}",
                        condition.to_string(),
                        consequence.iter().fold("".to_owned(), |acc, a| acc
                            + &" ".to_owned()
                            + &a.to_string())[1..]
                            .to_owned(),
                        alternative.iter().fold("".to_owned(), |acc, a| acc
                            + &" ".to_owned()
                            + &a.to_string())[1..]
                            .to_owned()
                    )
                }
                else {
                    format!(
                        "if({}) {{ {} }}",
                        condition.to_string(),
                        consequence.iter().fold("".to_owned(), |acc, a| acc
                            + &" ".to_owned()
                            + &a.to_string())[1..]
                            .to_owned()
                    )
                }
            }
            Self::FunctionLiteral {
                token: _,
                parameters,
                body,
            } => {
                format!(
                    "fn({}) {{ {} }}",
                    parameters
                        .iter()
                        .fold("".to_owned(), |acc, a| acc + &", ".to_owned() + &a.value)[2..]
                        .to_owned(),
                    body.iter().fold("".to_owned(), |acc, a| acc
                        + &", ".to_owned()
                        + &a.to_string())[2..]
                        .to_owned()
                )
            }
            Self::FunctionCall {
                token: _,
                func,
                args,
            } => {
                format!(
                    "{}({})",
                    func.to_string(),
                    args.iter().fold("".to_owned(), |acc, a| acc
                        + &", ".to_owned()
                        + &a.to_string())[2..]
                        .to_owned()
                )
            }
            _ => format!("Undefined"),
        }
    }
}

#[derive(Debug, PartialEq)]
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
impl Node for Program {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Program {
    pub fn empty(&self) -> bool {
        self.statements.is_empty()
    }
}
