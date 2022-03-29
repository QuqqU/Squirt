use std::collections::HashMap;

pub struct Env {
    map: HashMap<String, Box<dyn super::Object>>,
}
impl Env {
    pub fn new() -> Self {
        Self {
            map: HashMap::<String, Box<dyn super::Object>>::new(),
        }
    }

    pub fn get(&self, s: &String) -> Option<&Box<dyn super::Object>> {
        self.map.get(s)
    }

    pub fn set(&mut self, key: String, value: Box<dyn super::Object>) {
        self.map.insert(key, value);
    }
}
