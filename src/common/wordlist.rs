use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load_common_passwords(path: &str) -> HashSet<String> {
    let file = File::open(path).expect("Failed to open common passwords file.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .collect()
}
