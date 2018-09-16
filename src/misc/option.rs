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
    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
    pub fn get(&self, key: &str) -> &String {
        &self.map[key]
    }
    pub fn flush(&mut self) {
        self.map.insert(
            self.buffer_name.to_string(),
            self.buffer_value.to_string()
        );
    }
}