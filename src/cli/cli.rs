use std::collections::HashMap;
use serde::Deserialize;
use std::fs;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub adjacency_rules: HashMap<String, Vec<String>>,
    pub tiles: HashMap<String, String>
}

impl Config {
    pub fn from_yaml(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_str = fs::read_to_string(file_path)?;
        let config: Config = serde_yaml::from_str(&yaml_str)?;
        Ok(config)
    }
}