use super::stmt::BlockStmts;
use crate::Location;

use std::fmt;

#[derive(PartialEq, Clone)]
pub enum PrefixType {
    Minus,
    Bang,
}

#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
pub struct Params(pub Vec<Expr>);

#[derive(PartialEq, Clone)]
pub struct Args(pub Vec<Expr>);

#[derive(PartialEq, Clone)]
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

impl fmt::Debug for PrefixType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
        }
    }
}

impl fmt::Debug for InfixType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign => write!(f, "="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::Eq => write!(f, "=="),
            Self::Neq => write!(f, "!="),
        }
    }
}

impl fmt::Debug for Params {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = &self.0;
        let mut s = String::from("");
        for (idx, expr) in v.iter().enumerate() {
            if idx > 0 {
                s += ", ";
            }
            s += &format!("{:?}", expr);
        }
        write!(f, "({})", s)
    }
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = &self.0;
        let mut s = String::from("");
        for (idx, expr) in v.iter().enumerate() {
            if idx > 0 {
                s += ", ";
            }
            s += &format!("{:?}", expr);
        }
        write!(f, "({})", s)
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident { loc: _, name } => write!(f, "{}", name),
            Self::Int { loc: _, value } => write!(f, "{}", value),
            Self::Bool { loc: _, value } => write!(f, "{}", value),
            Self::Prefix {
                loc: _,
                operator,
                right,
            } => write!(f, "{:?}{:?}", operator, right),
            Self::Infix {
                loc: _,
                left,
                operator,
                right,
            } => write!(f, "{:?} {:?} {:?}", left, operator, right),
            Self::If {
                loc: _,
                condition,
                consequence,
                alternative,
            } => {
                if alternative.is_empty() {
                    write!(f, "if ({:?}) {:?}", condition, consequence)
                }
                else {
                    write!(
                        f,
                        "if ({:?}) {:?} else {:?}",
                        condition, consequence, alternative
                    )
                }
            }
            Self::FuncLiteral {
                loc: _,
                parameters,
                body,
            } => write!(f, "fn {:?} {:?}", parameters, body),
            Self::FuncCall {
                loc: _,
                ident,
                args,
            } => write!(f, "{:?}{:?}", ident, args),
        }
    }
}

#[cfg(test)]
mod fmt_dbg {
    use crate::{Args, Expr, InfixType, Location, Params};

    #[test]
    fn params_empty() {
        let expected = "()";
        let dbg_str = format!("{:?}", Params(vec![]));
        assert_eq!(expected, dbg_str);
    }

    #[test]
    fn params_singleton() {
        let expected = "(a)";
        let dbg_str = format!(
            "{:?}",
            Params(vec![Expr::Ident {
                loc:  Location::new(0, 0),
                name: "a".to_string(),
            }])
        );
        assert_eq!(expected, dbg_str);
    }
    #[test]
    fn params() {
        let expected = "(a, b, c)";
        let dbg_str = format!(
            "{:?}",
            Params(vec![
                Expr::Ident {
                    loc:  Location::new(0, 0),
                    name: "a".to_string(),
                },
                Expr::Ident {
                    loc:  Location::new(0, 0),
                    name: "b".to_string(),
                },
                Expr::Ident {
                    loc:  Location::new(0, 0),
                    name: "c".to_string(),
                }
            ])
        );
        assert_eq!(expected, dbg_str);
    }

    #[test]
    fn args_empty() {
        let expected = "()";
        let dbg_str = format!("{:?}", Args(vec![]));
        assert_eq!(expected, dbg_str);
    }

    #[test]
    fn args_singleton() {
        let expected = "(a)";
        let dbg_str = format!(
            "{:?}",
            Args(vec![Expr::Ident {
                loc:  Location::new(0, 0),
                name: "a".to_string(),
            }])
        );
        assert_eq!(expected, dbg_str);
    }

    #[test]
    fn args() {
        let expected = "(a, 2, c + 3)";
        let dbg_str = format!(
            "{:?}",
            Args(vec![
                Expr::Ident {
                    loc:  Location::new(0, 0),
                    name: "a".to_string(),
                },
                Expr::Int {
                    loc:   Location::new(0, 0),
                    value: 2,
                },
                Expr::Infix {
                    loc:      Location::new(0, 0),
                    left:     Box::new(Expr::Ident {
                        loc:  Location::new(0, 0),
                        name: "c".to_string(),
                    }),
                    operator: InfixType::Plus,
                    right:    Box::new(Expr::Int {
                        loc:   Location::new(0, 0),
                        value: 3,
                    }),
                },
            ])
        );
        assert_eq!(expected, dbg_str);
    }
}
