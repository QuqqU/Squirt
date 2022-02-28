// use std::fmt::*;

use token::TokenType;

// #[derive(Debug)]
pub struct Identifier {
    pub token: TokenType, // token::IDENT
    pub value: String,
}

// #[derive(Debug)]
pub enum Expression {
    Undefined,
    Ident(Identifier),
}
// impl Debug for Expression {}

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
        token:      TokenType,
        expression: Expression,
    },
}
// impl Debug for Statement {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self {
//             Self::Let { token, name, value } => write!(f, "let {} = {:?};", name.value, value),
//             Self::Return { token, value } => write!(f, "return {:?};", value),
//             Self::Expr { token, expression } => write!(f, "{:?};", expression),
//         }
//     }
// }

pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn empty(&self) -> bool {
        self.statements.is_empty()
    }
}
