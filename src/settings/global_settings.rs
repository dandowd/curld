use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use dirs;

#[derive(Serialize, Deserialize)]
pub struct GlobalSettings {
    #[serde(default)]
    pub working_dir: String, 

    pub module_settings: HashMap<String, Value>
}
