//mod util;

// use std::any::Any;

// use lexer::loc::Location;

use super::location::Location;

// #[derive(Debug, PartialEq, Clone)]
// pub struct Identifier {
//     pub loc: TokenType, // loc::IDENT
//     pub value: String,
// }
// pub type BlockStatement = Vec<Stmt>;

// pub trait Node {
//     fn as_any(&self) -> &dyn Any;
// }

#[derive(Debug, PartialEq, Clone)]
pub enum PrefixType {
    Minus,
    Bang,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfixType {
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    Neq,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Poison,
    Ident {
        loc:  Location,
        name: String,
    },
    Int {
        loc:   Location,
        value: i64,
    },
    Bool {
        loc:   Location,
        value: bool,
    },
    Prefix {
        loc:      Location,
        operator: PrefixType,
        right:    Box<Expr>,
    },
    Infix {
        loc:      Location,
        left:     Box<Expr>,
        operator: InfixType,
        right:    Box<Expr>,
    },
    If {
        loc:         Location,
        condition:   Box<Expr>,
        consequence: Vec<Stmt>,
        alternative: Vec<Stmt>,
    },
    FunctionLiteral {
        loc:        Location,
        parameters: Vec<Expr>, // Expr::Ident
        body:       Vec<Stmt>,
    },
    FunctionCall {
        loc:  Location,  // loc::LPAREN
        func: Box<Expr>, // functionliteral or ident
        args: Vec<Expr>,
    },
}
// impl Node for Expr {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Let {
        // loc:   Location, // loc::LET
        name:  Expr, // Expr::Ident
        value: Expr,
    },
    Return {
        // loc:   Location, // loc::RETURN
        value: Expr,
    },
    Expr {
        // loc:        Location, //expression 의 첫 토큰
        expression: Expr,
    },
}
// impl Node for Stmt {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
// impl Node for Program {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
