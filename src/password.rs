use colored::Colorize;
use rand::prelude::*;
use regex::Regex;

use crate::{common::load_common_passwords, utils::*};

pub const DEFAULT_LENGTH: usize = 16;
pub const MIN_LENGTH: usize = 10;

pub fn has_upper(password: &str) -> bool {
    Regex::new(r"[A-Z]").unwrap().is_match(password)
}

pub fn has_lower(password: &str) -> bool {
    Regex::new(r"[a-z]").unwrap().is_match(password)
}

pub fn has_number(password: &str) -> bool {
    Regex::new(r"\d").unwrap().is_match(password)
}

pub fn has_symbol(password: &str) -> bool {
    Regex::new(r"[^\w]").unwrap().is_match(password)
}

pub fn valid_length(password: &str) -> bool {
    password.len() >= MIN_LENGTH
}

pub fn generate_password(length: usize) -> String {
    let mut rng = rand::rng();

    let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let digits: Vec<char> = "0123456789".chars().collect();
    let specials: Vec<char> = "!@#$%^&*()-_+=.".chars().collect();

    let mut all_chars = lowercase.clone();
    all_chars.extend(&uppercase);
    all_chars.extend(&digits);
    all_chars.extend(&specials);

    let mut password = vec![
        *lowercase.choose(&mut rng).unwrap(),
        *uppercase.choose(&mut rng).unwrap(),
        *digits.choose(&mut rng).unwrap(),
        *specials.choose(&mut rng).unwrap(),
    ];

    for _ in 0..(length - 4) {
        password.push(*all_chars.choose(&mut rng).unwrap());
    }

    password.shuffle(&mut rng);
    password.into_iter().collect()
}

pub fn score_password(password: &str) -> u8 {
    let mut score: u8 = 0;

    if valid_length(password) {
        score += 1;
    }

    if has_upper(password) {
        score += 1;
    }

    if has_lower(password) {
        score += 1;
    }

    if has_number(password) {
        score += 1;
    }

    if has_symbol(password) {
        score += 1;
    }

    if score < 5 {
        print_suggestions(password);
    }

    score
}

pub fn analyze(password: &str, common_file: &Option<String>) {
    let common_passwords = common_file.as_ref().map(|path| load_common_passwords(path));

    if let Some(set) = common_passwords {
        if set.contains(password) {
            println!("{}", "⚠️ This password is too common!".red().bold());
            print_strength_bar(0);
            return;
        }
    }

    let score = score_password(password);
    print_strength_bar(score);
}
