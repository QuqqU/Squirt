use super::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct Env {
    inner: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Env>>>,
}
impl Env {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<String, Object>::new(),
            outer: Option::None,
        }
    }

    pub fn wrap_env(outer: Env) -> Self {
        Self {
            inner: HashMap::<String, Object>::new(),
            outer: Some(Rc::new(RefCell::new(outer))),
        }
    }

    pub fn get(&self, s: &str) -> Option<Object> {
        let obj = self.inner.get(s);
        match obj {
            Some(v) => Some(v.clone()),
            None => {
                if let Some(o) = &self.outer {
                    o.borrow().get(s)
                }
                else {
                    None
                }
            }
        }
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.inner.insert(key, value);
    }
}

#[cfg(test)]
mod env {
    use crate::Env;
    use crate::Object;

    fn init() -> Env {
        // inject var in outer-env
        let mut env = Env::new();
        env.set("out".to_string(), Object::Boolean(true));

        // inject var in inner-env
        let mut env = Env::wrap_env(env);
        env.set("in".to_string(), Object::Boolean(false));

        env
    }

    #[test]
    fn exist_inner() {
        let env = init();
        let is_exist = env.get("in");
        match is_exist {
            Some(obj) => assert_eq!(obj, Object::Boolean(false)),
            None => panic!("OBJ:0011"),
        }
    }

    #[test]
    fn exist_outer() {
        let env = init();
        let is_exist = env.get("out");
        match is_exist {
            Some(obj) => assert_eq!(obj, Object::Boolean(true)),
            None => panic!("OBJ:0012"),
        }
    }

    #[test]
    fn not_exist() {
        let env = init();
        let is_exist = env.get("not");
        match is_exist {
            Some(_) => panic!("OBJ:0013"),
            None => {}
        }
    }
}
