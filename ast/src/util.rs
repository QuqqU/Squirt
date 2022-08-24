use crate::node::*;
use std::fmt;

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

impl BlockStmts {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Debug for BlockStmts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "{{}}");
        }

        let mut s = String::from("");
        for (idx, expr) in self.0.iter().enumerate() {
            if idx > 0 {
                s += " ";
            }
            s += &format!("{:?}", expr);
        }
        write!(f, "{{ {} }}", s)
    }
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

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let { name, expr } => write!(f, "let {:?} = {:?};", name, expr),
            Self::Return { expr } => write!(f, "return {:?};", expr),
            Self::Expr { expr } => write!(f, "{:?};", expr),
        }
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.stmts)
    }
}

#[cfg(test)]
mod fmt_dbg {
    use crate::{Args, BlockStmts, Expr, InfixType, Location, Params, Stmt};

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

    #[test]
    fn block_stmts_empty() {
        let expected = "{}";
        let dbg_str = format!("{:?}", BlockStmts(vec![]));
        assert_eq!(expected, dbg_str);
    }

    #[test]
    fn block_stmts_singleton() {
        let expected = "{ return a; }";
        let dbg_str = format!(
            "{:?}",
            BlockStmts(vec![Stmt::Return {
                expr: Expr::Ident {
                    loc:  Location::new(0, 0),
                    name: "a".to_string(),
                },
            }])
        );
        assert_eq!(expected, dbg_str);
    }

    #[test]
    fn block_stmts() {
        let expected = "{ return a; return b; }";
        let dbg_str = format!(
            "{:?}",
            BlockStmts(vec![
                Stmt::Return {
                    expr: Expr::Ident {
                        loc:  Location::new(0, 0),
                        name: "a".to_string(),
                    },
                },
                Stmt::Return {
                    expr: Expr::Ident {
                        loc:  Location::new(0, 0),
                        name: "b".to_string(),
                    },
                }
            ])
        );
        assert_eq!(expected, dbg_str);
    }
}
