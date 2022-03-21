use std::any::Any;
use std::fmt::Debug;

pub type ObjectType = &'static str;

pub trait Object: Debug {
    fn as_any(&self) -> &dyn Any;
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

#[derive(Debug)]
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
