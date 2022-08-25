use super::stmt::Stmt;

use std::fmt;

pub struct Program {
    pub program: Vec<Stmt>,
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.program)
    }
}
