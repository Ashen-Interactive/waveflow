use std::{collections::HashMap, path::Path};
use serde::Deserialize;
use serde_yaml;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Sector {
    pub path: String,
    pub dimensions: (u32, u32), // width, height
    pub origin: (u32, u32),     // x, y
}

#[derive(Debug, Deserialize)]
pub struct Tile {
    pub name: String,
    pub path: String,
    pub description: String,
    pub color_code: String,     // Hex color code
    pub dimensions: (u32, u32), // width, height

    pub sectors: Vec<Sector>,

    // pub adjacency_rules: HashMap<String, Vec<String>>, // Adjacency rules for WFC
}

impl Tile {
    pub fn from_yaml(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_str = fs::read_to_string(file_path)?;
        let mut tile: Tile = serde_yaml::from_str(&yaml_str)?;

        // Get the directory of the YAML file
        let base_dir = Path::new(file_path)
            .parent()
            .unwrap_or_else(|| Path::new(""));

        // Fix each sector path to be absolute or relative to the YAML file
        for sector in &mut tile.sectors {
            let fixed_path = base_dir.join(&sector.path);
            sector.path = fixed_path.to_string_lossy().to_string();
        }

        Ok(tile)
    }
}
