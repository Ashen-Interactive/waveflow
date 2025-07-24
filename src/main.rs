use std::{fs, path::PathBuf};
use clap::Parser;
use anyhow::Result;
use image::RgbaImage;

mod config;
mod map;

use config::MapConfig;
use map::{generate_map_1d, generate_map_2d};

#[derive(Parser, Debug)]
#[command(name = "waveflow", version = "0.1.0", about = "1D WFC Map Concatenator")]
struct Args {
    /// Path to map.yaml
    #[arg(long)]
    map: PathBuf,

    #[arg(long)]
    dimensions: Option<String>,

    /// Directory holding tile PNGs (named `<name>.png`)
    #[arg(long)]
    tiles_dir: PathBuf,

    /// Number of tiles in the output (length of the 1D map)
    #[arg(long, default_value_t = 4)]
    length: usize,

    /// Output PNG file
    #[arg(long)]
    out: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load adjacency rules
    let cfg = MapConfig::load(&args.map)?;

    // Gather all `.png` tile paths
    let mut tile_paths = Vec::new();
    for entry in fs::read_dir(&args.tiles_dir)? {
        let p = entry?.path();
        if p.extension().and_then(|s| s.to_str()) == Some("png") {
            tile_paths.push(p);
        }
    }

    // Build the map

    if let Some(dimensions) = &args.dimensions {
        let result: RgbaImage = generate_map_2d(&tile_paths, &cfg.adjacency_rules, args.length, args.length)?;
        // Save the output
        result.save(&args.out)?;
        println!("Generated 1D map: {} tiles → {}", args.length, args.out.display());
    } else {
        let result: RgbaImage = generate_map_1d(&tile_paths, &cfg.adjacency_rules, args.length)?;
        // Save the output
        result.save(&args.out)?;
        println!("Generated 1D map: {} tiles → {}", args.length, args.out.display());
    }

    Ok(())
}
