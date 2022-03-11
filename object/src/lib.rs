pub type ObjectType = String;

pub trait Object {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

pub struct Null {}
impl Object for Null {
    fn object_type(&self) -> ObjectType {
        "Null".to_owned()
    }
    fn inspect(&self) -> String {
        format!("null")
    }
}
pub const NULL: Null = Null {};

pub struct Integer {
    pub value: i64,
}
impl Object for Integer {
    fn object_type(&self) -> ObjectType {
        "Integer".to_owned()
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

pub type Bool = &'static bool;
impl Object for Bool {
    fn object_type(&self) -> ObjectType {
        "Bool".to_owned()
    }
    fn inspect(&self) -> String {
        format!("{}", self)
    }
}
const TRUE: Bool = &true;
const FALSE: Bool = &false;
pub fn static_bool_obj(b: bool) -> Bool {
    if b {
        TRUE
    }
    else {
        FALSE
    }
}
