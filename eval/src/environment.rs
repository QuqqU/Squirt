use std::collections::HashMap;

pub struct Env {
    map: HashMap<String, Box<dyn object::Object>>,
}
impl Env {
    pub fn new() -> Self {
        Self {
            map: HashMap::<String, Box<dyn object::Object>>::new(),
        }
    }

    pub fn get(&self, s: &String) -> Option<&Box<dyn object::Object>> {
        self.map.get(s)
    }

    pub fn set(&mut self, key: String, value: Box<dyn object::Object>) {
        self.map.insert(key, value);
    }
}
