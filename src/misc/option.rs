use std::collections::HashMap;

pub struct EngineSettings {
    pub map: HashMap<String, String>,
    pub buffer_name: String,
    pub buffer_value: String,
}
impl EngineSettings {
    pub fn new() -> EngineSettings {
        EngineSettings {
            map: HashMap::new(),
            buffer_name: "".to_string(),
            buffer_value: "".to_string(),
        }
    }
    pub fn contains(&self, key: &String) -> bool {
        self.map.contains_key(key)
    }
    pub fn get(&self, key: &String) -> &String {
        self.map.get(key).unwrap()
    }
    pub fn flush(&mut self) {
        self.map.insert(
            self.buffer_name.to_string(),
            self.buffer_value.to_string()
        );
    }
}