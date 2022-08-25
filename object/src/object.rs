use crate::environment::Env;
use ast::{self, BlockStmts, Params};

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub enum Object {
    Error(String),
    Null,
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Function {
        params: Params,
        body:   BlockStmts,
        env:    Rc<RefCell<Env>>,
    },
}
impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error(e) => write!(f, "{}", e),
            Self::Null => write!(f, "__NULL__"),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", *value == true),
            Self::ReturnValue(value) => write!(f, "{:?}", value),
            Self::Function {
                params,
                body,
                env: _,
            } => write!(f, "fn {:?} {:?}", params, body),
        }
    }
}
