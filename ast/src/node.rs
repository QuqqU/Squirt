use super::location::Location;

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
    FuncLiteral {
        loc:        Location,
        parameters: Vec<Expr>, // Vec<Expr::Ident>
        body:       Vec<Stmt>,
    },
    FuncCall {
        loc:   Location,  // loc of Lparen
        ident: Box<Expr>, //Expr::FuncLiteral
        args:  Vec<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Let {
        name: Expr, // Expr::Ident
        expr: Expr,
    },
    Return {
        expr: Expr,
    },
    Expr {
        expr: Expr,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
