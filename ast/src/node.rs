use super::location::Location;

#[derive(PartialEq)]
pub struct Params(pub Vec<Expr>);

#[derive(PartialEq)]
pub struct Args(pub Vec<Expr>);

#[derive(PartialEq)]
pub struct BlockStmts(pub Vec<Stmt>);

#[derive(PartialEq)]
pub enum PrefixType {
    Minus,
    Bang,
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub enum Expr {
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
        consequence: BlockStmts,
        alternative: BlockStmts,
    },
    FuncLiteral {
        loc:        Location,
        parameters: Params, //Vec<Expr>, // Vec<Expr::Ident>
        body:       BlockStmts,
    },
    FuncCall {
        loc:   Location,  // loc of Lparen
        ident: Box<Expr>, //Expr::FuncLiteral
        args:  Args,
    },
}

#[derive(PartialEq)]
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

pub struct Program {
    pub program: Vec<Stmt>,
}
