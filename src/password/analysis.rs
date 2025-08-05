use crate::common::wordlist::load_common_passwords;
use crate::utils::printer::{print_strength_bar, print_suggestions};
use colored::Colorize;
use regex::Regex;

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

fn score_password(password: &str) -> u8 {
    let mut score: u8 = 0;

    let length = if valid_length(password) {
        score += 1;
        "✓".green()
    } else {
        "✘".red()
    };

    let upper = if has_upper(password) {
        score += 1;
        "✓".green()
    } else {
        "✘".red()
    };

    let lower = if has_lower(password) {
        score += 1;
        "✓".green()
    } else {
        "✘".red()
    };

    let number = if has_number(password) {
        score += 1;
        "✓".green()
    } else {
        "✘".red()
    };

    let symbol = if has_symbol(password) {
        score += 1;
        "✓".green()
    } else {
        "✘".red()
    };

    println!();
    println!("{} Valid length (>={})", length, MIN_LENGTH);
    println!("{} Has uppercase", upper);
    println!("{} Has lowercase", lower);
    println!("{} Has numbers", number);
    println!("{} Has symbols", symbol);

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
