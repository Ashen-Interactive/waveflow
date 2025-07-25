use rand::prelude::IndexedRandom;
use std::collections::{HashMap, HashSet};

use crate::cli::cli::Config;

pub fn generate_map_array(_config: &Config) {
    // Define adjacency rules
    let rules: HashMap<&str, Vec<&str>> = HashMap::from([
        ("A", vec!["B"]),
        ("B", vec!["A", "C"]),
        ("C", vec!["A"]),
    ]);

    // All possible states (as a slice)
    let possible_states = ["A", "B", "C"];

    // Create the wave: each cell starts with all possible states
    let mut wave: Vec<HashSet<&str>> = vec![possible_states.iter().copied().collect(); 10];

    // Collapse the first cell randomly
    let first_choice = *possible_states.choose(&mut rand::thread_rng()).unwrap();
    wave[0] = HashSet::from([first_choice]);

    // Collapse the rest based on adjacency rules
    for i in 1..wave.len() {
        let prev = wave[i - 1].iter().next().unwrap();
        let allowed = &rules[prev];
        let chosen = *allowed.choose(&mut rand::thread_rng()).unwrap();
        wave[i] = HashSet::from([chosen]);
    }

    // Convert collapsed wave into a final result
    let result: Vec<&str> = wave.into_iter().map(|s| *s.iter().next().unwrap()).collect();

    println!("Collapsed sequence: {:?}", result);
}
