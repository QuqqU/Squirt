mod environment;
pub use environment::*;

use ast;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub type ObjectType = &'static str;

pub trait Object: Debug + BoxClone {
    fn as_any(&self) -> &dyn Any;
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

//https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait BoxClone {
    fn clone_box(&self) -> Box<dyn Object>;
}
impl<T> BoxClone for T
where
    T: 'static + Object + Clone,
{
    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Object> {
    fn clone(&self) -> Box<dyn Object> {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct Never {}
impl Object for Never {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        "Never"
    }
    fn inspect(&self) -> String {
        format!("never_inspected")
    }
}

#[derive(Debug, Clone)]
pub struct Null {}
impl Object for Null {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        "Null"
    }
    fn inspect(&self) -> String {
        format!("null")
    }
}
pub const NULL: Null = Null {};

#[derive(Debug, Clone, Copy)]
pub struct Integer {
    pub value: i64,
}
impl Object for Integer {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        "Integer"
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bool {
    pub value: &'static bool,
}
impl Object for Bool {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        "Bool"
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}
pub const TRUE: Bool = Bool { value: &true };
pub const FALSE: Bool = Bool { value: &false };
pub fn static_bool_obj(b: bool) -> Bool {
    if b {
        TRUE
    }
    else {
        FALSE
    }
}

#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub value: Box<dyn Object>,
}
impl Object for ReturnValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        "ReturnValue"
    }
    fn inspect(&self) -> String {
        format!("{}", self.value.inspect())
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub value: String,
}
impl Object for Error {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        "Error"
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<ast::Identifier>,
    pub body:       Vec<ast::Statement>,
    pub env:        Rc<RefCell<Env>>,
}
impl Object for Function {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        "Funciton"
    }
    fn inspect(&self) -> String {
        format!("fn() {{}}") // todo
    }
}
