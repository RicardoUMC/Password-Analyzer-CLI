use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use colored::*;
use regex::Regex;

#[derive(Parser)]
#[command(name = "passcheck")]
#[command(about = "Analize password strength", long_about = None)]
struct Args {
    password: String,

    #[arg(short, long)]
    common: Option<String>,
}

fn has_upper(password: &str) -> bool {
    Regex::new(r"[A-Z]").unwrap().is_match(password)
}

fn has_lower(password: &str) -> bool {
    Regex::new(r"[a-z]").unwrap().is_match(password)
}

fn has_number(password: &str) -> bool {
    Regex::new(r"\d").unwrap().is_match(password)
}

fn has_symbol(password: &str) -> bool {
    Regex::new(r"[^\w]").unwrap().is_match(password)
}

fn valid_length(password: &str) -> bool {
    password.len() >= 10
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
    println!("{} Valid length (>=10)", length);
    println!("{} Has uppercase", upper);
    println!("{} Has lowercase", lower);
    println!("{} Has numbers", number);
    println!("{} Has symbols", symbol);

    if score < 5 {
        print_suggestions(password);
    }

    score
}

fn print_suggestions(password: &str) {
    println!("\n{}", "Suggestions to improve your password:".yellow());

    if !valid_length(password) {
        println!("{}", "- Make it at least 10 characters long.".yellow());
    }
    if !has_upper(password) {
        println!("{}", "- Include at least one uppercase letter.".yellow());
    }
    if !has_lower(password) {
        println!("{}", "- Include at least one lowercase letter.".yellow());
    }
    if !has_number(password) {
        println!("{}", "- Include at least one number.".yellow());
    }
    if !has_symbol(password) {
        println!("{}", "- Include at least one special character (e.g., !@#$%^&*).".yellow());
    }
}

fn print_strength_bar(score: u8) {
    let total_block = 10;
    let filled = score * 2;
    let empty = total_block - filled;

    let bar = format! {
        "[{}{}]",
        "■".repeat(filled as usize),
        "-".repeat(empty as usize)
    };

    let (color_bar, label) = match score {
        0..=2 => (bar.red(), "Weak".red()),
        3 => (bar.yellow(), "Medium".yellow()),
        4..=5 => (bar.green(), "Strong".green()),
        _ => (bar.normal(), "Unknown".normal()),
    };

    let percentage = score * 20;
    println!(
        "\nPassowrd strength: {} {}% ({})",
        color_bar, percentage, label
    );
}

fn load_common_passwords(path: &str) -> HashSet<String> {
    let file = File::open(path).expect("Failed to open common passwords file.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .collect()
}

fn main() {
    let args = Args::parse();
    let password = &args.password;
    println!("Received password: {}", password);

    println!("--- Security Analysis ---");

    let common_passwords = args.common.as_ref().map(|path| load_common_passwords(path));

    if let Some(list) = common_passwords {
        match list.contains(password) {
            true => {
                println!("{}", "⚠️ This password is too common!".red().bold());
                print_strength_bar(0);
            }
            false => {
                let score = score_password(password);
                print_strength_bar(score);
            }
        }
    } else {
        let score = score_password(password);
        print_strength_bar(score);
    }
}
