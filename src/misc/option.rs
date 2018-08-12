use std::collections::HashMap;

pub struct Option {
    pub name: String,
    pub type_name: String,
    pub default_number: i64,
    pub default_string: String,
    pub min: i64,
    pub max: i64,
}

pub struct EngineSettings {
    pub map: HashMap<String, Option>,
    pub buffer_name: String,
    pub buffer_type_name: String,
    pub buffer_default: String,
    pub buffer_min: i64,
    pub buffer_max: i64,
}
impl EngineSettings {
    pub fn new() -> EngineSettings {
        EngineSettings {
            map: HashMap::new(),
            buffer_name: "".to_string(),
            buffer_type_name: "".to_string(),
            buffer_default: "".to_string(),
            buffer_min: 0,
            buffer_max: 0,
        }
    }
    pub fn register_name(&mut self, name: String){
        self.buffer_name = name.to_string();
    }
    pub fn register_type(&mut self, type_name: String){
        self.buffer_type_name = type_name.to_string();
    }
    pub fn flush(&mut self) {
        let type_name : String = self.buffer_type_name.to_string();
        match &*type_name {
            "spin" => {
                // 数値型
                self.map.insert(
                    self.buffer_name.to_string(),
                    Option {
                        name: self.buffer_name.to_string(),
                        type_name: self.buffer_type_name.to_string(),
                        default_number: self.buffer_default.parse::<i64>().unwrap(),
                        default_string: "".to_string(),
                        min: self.buffer_min,
                        max: self.buffer_max,
                    }
                );
            },
            _ => {
                // 文字列型
                self.map.insert(
                    self.buffer_name.to_string(),
                    Option {
                        name: self.buffer_name.to_string(),
                        type_name: self.buffer_type_name.to_string(),
                        default_number: 0,
                        default_string: self.buffer_default.to_string(),
                        min: self.buffer_min,
                        max: self.buffer_max,
                    }
                );
            },
        };

    }
}