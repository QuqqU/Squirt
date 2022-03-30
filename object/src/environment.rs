use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct Env {
    inner: HashMap<String, Box<dyn super::Object>>,
    outer: Option<Rc<RefCell<Env>>>,
}
impl Env {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<String, Box<dyn super::Object>>::new(),
            outer: Option::None,
        }
    }

    pub fn wrap_env(outer: Rc<RefCell<Env>>) -> Self {
        Self {
            inner: HashMap::<String, Box<dyn super::Object>>::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, s: &String) -> Option<Box<dyn super::Object>> {
        let obj = self.inner.get(s);
        // println!("--> {} {:?} {:?}", s, &self.inner, &self.outer);
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

    pub fn set(&mut self, key: String, value: Box<dyn super::Object>) {
        self.inner.insert(key, value);
    }
}
impl Debug for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = match &self.outer {
            Some(v) => v.borrow_mut().inner.clone().into_keys().collect(),
            None => vec![],
        };
        f.debug_struct("Env")
            .field("inner", &self.inner.clone().into_keys().collect::<String>())
            .field("outer", &v)
            .finish()
    }
}
