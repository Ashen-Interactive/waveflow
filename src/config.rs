use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct MapConfig {
    #[serde(rename = "adjacency_rules")]
    pub adjacency_rules: HashMap<String, Vec<String>>,
}

impl MapConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)?;
        let cfg: MapConfig = serde_yaml::from_reader(file)?;
        Ok(cfg)
    }
}