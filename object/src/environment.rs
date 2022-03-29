use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Env {
    inner: HashMap<String, Box<dyn super::Object>>,
    outer: Option<Box<Env>>,
}
impl Env {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<String, Box<dyn super::Object>>::new(),
            outer: Option::None,
        }
    }

    pub fn wrap_env(outer: Box<Env>) -> Self {
        Self {
            inner: HashMap::<String, Box<dyn super::Object>>::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, s: &String) -> Option<&Box<dyn super::Object>> {
        let obj = self.inner.get(s);
        match obj {
            None => match &self.outer {
                Some(o) => o.get(s),
                None => None,
            },
            _ => obj,
        }
    }

    pub fn set(&mut self, key: String, value: Box<dyn super::Object>) {
        self.inner.insert(key, value);
    }
}
impl Debug for Env {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
