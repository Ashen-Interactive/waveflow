use std::collections::HashMap;
use serde::Deserialize;
use serde_yaml;
use std::fs;

// Sector for the sector yaml file
#[derive(Debug, Deserialize)]
pub struct Sector {
    pub name: String,
    pub color_code: String,
}

impl Sector {
    pub fn from_yaml(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_str = fs::read_to_string(file_path)?;
        let tile: Sector = serde_yaml::from_str(&yaml_str)?;
        Ok(tile)
    }
}
