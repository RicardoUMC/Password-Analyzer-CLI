use clap::Parser;
use colored::*;
use regex::Regex;

#[derive(Parser)]
#[command(name = "passcheck")]
#[command(about = "Analize password strength", long_about = None)]
struct Args {
    password: String,
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

    score
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

fn main() {
    let args = Args::parse();
    let password = &args.password;
    println!("Received password: {}", password);

    println!("--- Security Analysis ---");

    let score = score_password(password);
    print_strength_bar(score);
}
