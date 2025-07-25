use std::{fs::File, io::{BufReader, BufWriter}};

use crate::rules::{sector, tile::Tile};
use image::{ImageBuffer, ImageEncoder, Rgba, RgbaImage};

/// Convert a hex string like "#FF00AA" or "FF00AA" to an RGBA pixel
fn hex_to_rgba(hex: &str) -> Result<Rgba<u8>, String> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 && hex.len() != 8 {
        return Err("Hex color must be 6 or 8 characters long".into());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component")?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component")?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component")?;
    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid alpha component")?
    } else {
        255
    };

    Ok(Rgba([r, g, b, a]))
}

/// Generate the sector
pub fn generate_sectors(tile: &Tile, image: &mut RgbaImage) -> Result<(), String> {
    for sector_meta in &tile.sectors {
        // Load the sector YAML (sector::Sector)
        let file = File::open(&sector_meta.path)
            .map_err(|e| format!("Failed to open sector file {}: {}", sector_meta.path, e))?;
        let reader = BufReader::new(file);
        let sector_data: sector::Sector = serde_yaml::from_reader(reader)
            .map_err(|e| format!("Failed to parse sector YAML: {}", e))?;

        let rgba = hex_to_rgba(&sector_data.color_code)?;

        let (w, h) = sector_meta.dimensions;
        let (ox, oy) = sector_meta.origin;

        for y in 0..h {
            for x in 0..w {
                if ox + x < tile.dimensions.0 && oy + y < tile.dimensions.1 {
                    image.put_pixel(ox + x, oy + y, rgba);
                }
            }
        }
    }

    Ok(())
}

/// Generate a solid color PNG image
pub fn generate_image(tile: &Tile) -> Result<(), String> {
    let rgba = hex_to_rgba(&tile.color_code)?;
    let mut img: RgbaImage = ImageBuffer::from_fn(tile.dimensions.0, tile.dimensions.1, |_x, _y| rgba);

    // Fill sectors on top
    generate_sectors(tile, &mut img)?;

    let out_path = tile.path.replace(".yaml", ".png");
    let file = File::create(&out_path).map_err(|e| format!("File error: {}", e))?;
    let writer = BufWriter::new(file);
    image::codecs::png::PngEncoder::new(writer)
        .write_image(
            &img,
            tile.dimensions.0,
            tile.dimensions.1,
            image::ColorType::Rgba8.into(),
        )
        .map_err(|e| format!("Encoding error: {}", e))?;

    println!("Saved image to {}", out_path);
    Ok(())
}
