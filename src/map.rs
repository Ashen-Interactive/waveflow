use anyhow::{anyhow, Result};
use image::{GenericImage, RgbaImage};
use rand::seq::IteratorRandom;
use rand::rng;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::PathBuf,
};

/// Maximum number of full-restart attempts before giving up
const MAX_RETRIES: usize = 10;

/// Build a 1D WFC‐solved map of `length` tiles placed side‐by‐side,
/// retrying on contradictions up to `MAX_RETRIES` times.
pub fn generate_map_1d(
    tile_paths: &Vec<PathBuf>,
    adjacency_rules: &HashMap<String, Vec<String>>,
    length: usize,
) -> Result<RgbaImage> {
    // Load images once
    let tile_images = load_tile_images(tile_paths)?;
    let all_tiles: Vec<String> = adjacency_rules.keys().cloned().collect();

    // Try collapse up to MAX_RETRIES times
    for attempt in 1..=MAX_RETRIES {
        match try_collapse(&tile_images, adjacency_rules, &all_tiles, length) {
            Ok(canvas) => {
                println!("WFC succeeded on attempt {}/{}", attempt, MAX_RETRIES);
                return Ok(canvas);
            }
            Err(e) if e.to_string().starts_with("Contradiction") => {
                eprintln!("Attempt {}/{} failed: {} – restarting", attempt, MAX_RETRIES, e);
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    Err(anyhow!(
        "Failed to generate map after {} retries",
        MAX_RETRIES
    ))
}

/// One full collapse + stitching attempt
fn try_collapse(
    tile_images: &HashMap<String, RgbaImage>,
    adjacency_rules: &HashMap<String, Vec<String>>,
    all_tiles: &[String],
    length: usize,
) -> Result<RgbaImage> {
    // 1. Initialize possibilities
    let mut possibilities: Vec<HashSet<String>> =
        vec![all_tiles.iter().cloned().collect(); length];
    let mut queue = VecDeque::new();
    let mut rng = rng();

    // 2. Collapse + propagate
    while let Some((idx, choices)) = next_collapse_index(&possibilities) {
        // Pick randomly among choices
        let pick = choices
            .iter()
            .choose(&mut rng)
            .ok_or_else(|| anyhow!("No choices at idx {}", idx))?
            .clone();

        possibilities[idx].clear();
        possibilities[idx].insert(pick);
        queue.push_back(idx);

        // Propagate constraints
        while let Some(pos) = queue.pop_front() {
            for &nbr in &[pos.wrapping_sub(1), pos + 1] {
                if nbr >= length {
                    continue;
                }
                // Compute allowed neighbors
                let mut allowed = HashSet::new();
                for tile in &possibilities[pos] {
                    if let Some(adj) = adjacency_rules.get(tile) {
                        for t in adj {
                            allowed.insert(t.clone());
                        }
                    }
                }
                // Intersect
                let new_set: HashSet<String> = possibilities[nbr]
                    .drain()
                    .filter(|t| allowed.contains(t.as_str()))
                    .collect();

                if new_set.is_empty() {
                    return Err(anyhow!("Contradiction at index {}", nbr));
                }

                if new_set.len() < possibilities[nbr].len() {
                    possibilities[nbr] = new_set;
                    queue.push_back(nbr);
                } else {
                    // restore old if unchanged
                    possibilities[nbr] = new_set;
                }
            }
        }
    }

    // 3. Stitched assignment
    let assignment: Vec<String> = possibilities
        .into_iter()
        .map(|mut s| s.drain().next().unwrap())
        .collect();

    // 4. Create canvas
    let (tw, th) = {
        let img = tile_images.values().next().unwrap();
        (img.width(), img.height())
    };
    let mut canvas = RgbaImage::new(tw * (length as u32), th);
    for (i, name) in assignment.into_iter().enumerate() {
        let tile = tile_images
            .get(&name)
            .ok_or_else(|| anyhow!("Missing image for '{}'", name))?;
        canvas.copy_from(tile, (i as u32) * tw, 0)?;
    }

    Ok(canvas)
}

/// Helper: pick the cell with 2+ possibilities & smallest set
fn next_collapse_index(
    possibilities: &[HashSet<String>],
) -> Option<(usize, HashSet<String>)> {
    possibilities
        .iter()
        .enumerate()
        .filter(|(_, s)| s.len() > 1)
        .min_by_key(|(_, s)| s.len())
        .map(|(i, s)| (i, s.clone()))
}

pub fn generate_map_2d(
    tile_paths: &Vec<PathBuf>,
    adjacency_rules: &HashMap<String, Vec<String>>,
    width: usize,
    height: usize,
) -> Result<RgbaImage> {
    // Load images once
    let tile_images = load_tile_images(tile_paths)?;
    let all_tiles: Vec<String> = adjacency_rules.keys().cloned().collect();

    for attempt in 1..=MAX_RETRIES {
        match try_collapse_2d(&tile_images, adjacency_rules, &all_tiles, width, height) {
            Ok(canvas) => {
                println!("2D WFC succeeded on attempt {}/{}", attempt, MAX_RETRIES);
                return Ok(canvas);
            }
            Err(e) if e.to_string().starts_with("Contradiction") => {
                eprintln!("Attempt {}/{} failed: {} – restarting", attempt, MAX_RETRIES, e);
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    Err(anyhow!(
        "Failed to generate 2D map after {} retries",
        MAX_RETRIES
    ))
}

/// One full collapse + stitching attempt for 2D
fn try_collapse_2d(
    tile_images: &HashMap<String, RgbaImage>,
    adjacency_rules: &HashMap<String, Vec<String>>,
    all_tiles: &[String],
    width: usize,
    height: usize,
) -> Result<RgbaImage> {
    // 1. Initialize all cells to full domain
    let mut possibilities: Vec<Vec<HashSet<String>>> = vec![
        vec![all_tiles.iter().cloned().collect(); width];
        height
    ];
    let mut queue = VecDeque::new();
    let mut rng = rng();

    // 2. Collapse + propagate until everything is single-valued
    while let Some((x, y, choices)) = next_collapse_index_2d(&possibilities) {
        // Pick one tile at random
        let pick = choices
            .iter()
            .choose(&mut rng)
            .ok_or_else(|| anyhow!("No choices at ({}, {})", x, y))?
            .clone();

        possibilities[y][x].clear();
        possibilities[y][x].insert(pick);
        queue.push_back((x, y));

        // Propagate constraints in four directions
        while let Some((cx, cy)) = queue.pop_front() {
            for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if nx < 0 ||ny < 0 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if nx >= width || ny >= height {
                    continue;
                }

                // Compute allowed set for neighbor
                let mut allowed = HashSet::new();
                for tile in &possibilities[cy][cx] {
                    if let Some(adjs) = adjacency_rules.get(tile) {
                        for t in adjs {
                            allowed.insert(t.clone());
                        }
                    }
                }

                // Intersect neighbor's domain
                let old_len = possibilities[ny][nx].len();
                let new_set: HashSet<String> = possibilities[ny][nx]
                    .drain()
                    .filter(|t| allowed.contains(t))
                    .collect();

                if new_set.is_empty() {
                    return Err(anyhow!("Contradiction at ({}, {})", nx, ny));
                }
                possibilities[ny][nx] = new_set;

                // If it shrank, keep propagating
                if possibilities[ny][nx].len() < old_len {
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    // 3. Extract final assignment
    let assignment: Vec<Vec<String>> = possibilities
        .into_iter()
        .map(|row| row.into_iter().map(|mut s| s.drain().next().unwrap()).collect())
        .collect();

    // 4. Stitch into a single canvas
    let (tw, th) = {
        let img = tile_images.values().next().unwrap();
        (img.width(), img.height())
    };
    let mut canvas = RgbaImage::new(tw * (width as u32), th * (height as u32));

    for (y, row) in assignment.into_iter().enumerate() {
        for (x, name) in row.into_iter().enumerate() {
            let tile = tile_images
                .get(&name)
                .ok_or_else(|| anyhow!("Missing image for '{}'", name))?;
            canvas.copy_from(tile, (x as u32) * tw, (y as u32) * th)?;
        }
    }

    Ok(canvas)
}

/// Helper: pick the cell with 2+ possibilities & smallest domain in 2D
fn next_collapse_index_2d(
    possibilities: &[Vec<HashSet<String>>],
) -> Option<(usize, usize, HashSet<String>)> {
    let mut best: Option<(usize, usize, usize, HashSet<String>)> = None;
    for (y, row) in possibilities.iter().enumerate() {
        for (x, set) in row.iter().enumerate() {
            let len = set.len();
            if len > 1 {
                match &best {
                    Some((_, _, best_len, _)) if len >= *best_len => {}
                    _ => best = Some((x, y, len, set.clone())),
                }
            }
        }
    }
    best.map(|(x, y, _, choices)| (x, y, choices))
}

/// Load all PNGs into a name→image map
fn load_tile_images(
    tile_paths: &Vec<PathBuf>,
) -> Result<HashMap<String, RgbaImage>, anyhow::Error> {
    let mut map = HashMap::new();
    for path in tile_paths {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid file name"))?
            .to_string();
        let img = image::open(path)?.to_rgba8();
        map.insert(name, img);
    }
    Ok(map)
}
