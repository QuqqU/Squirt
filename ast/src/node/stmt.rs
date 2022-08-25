use super::expr::Expr;

use std::fmt;

#[derive(PartialEq, Clone)]
pub struct BlockStmts(pub Vec<Stmt>);

#[derive(PartialEq, Clone)]
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

impl BlockStmts {
    pub fn is_empty(&self) -> bool {
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

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let { name, expr } => write!(f, "let {:?} = {:?};", name, expr),
            Self::Return { expr } => write!(f, "return {:?};", expr),
            Self::Expr { expr } => write!(f, "{:?};", expr),
        }
    }
}

#[cfg(test)]
mod fmt_dbg {
    use crate::{BlockStmts, Expr, Location, Stmt};

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
