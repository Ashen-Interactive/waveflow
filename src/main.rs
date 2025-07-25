use std::fs;
use clap::{Arg, Command};
use image::{DynamicImage, GenericImage, GenericImageView, RgbaImage};

mod cli {
    pub mod cli;
}
mod generators {
    pub mod map;
    pub mod tile;
}
mod rules {
    pub mod sector;
    pub mod tile;
    pub mod wfc;
}

use cli::cli::*;
use rules::tile::*;
use rules::wfc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Decide what the user wants to make
    // 2. Generate the map firstly as an array of tiles (struct Tile(f64)) decided by maps.yaml (adjecensy rules) 
    // 3. Generate each tile as a section

    let matches = Command::new("waveflow")
        .version("0.1.0")
        .author("Neo Mannskär")
        .about("A tool for generating 2D game top-down maps through wave function collapse")
        .arg(
            Arg::new("input")
                .help("Input .yaml file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Output file generated map")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input = matches.get_one::<String>("input").expect("Input .yaml file is required");
    let output = matches.get_one::<String>("output").expect("Output .png file is required");
    let verbose = matches.get_flag("verbose");

    println!("Input file: {}", input);
    println!("Output file: {}", output);
    if verbose {
        println!("Verbose mode is enabled");
    }

    let tile: Tile = Tile::from_yaml(input)?;

    println!("{:?}", tile);

    generators::tile::generate_image(&tile)?;

    Ok(())
}
