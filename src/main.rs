use clap::Parser;
use colored::*;
use rand::prelude::*;
use rand::rng;
use regex::Regex;
use std::process;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser)]
#[command(name = "passcheck")]
#[command(about = "Analize password strength", long_about = None)]
struct Args {
    #[arg(help = "Password to check", required = false)]
    password: Option<String>,

    #[arg(short, long, help = "Path to list (wordlist) of common passowrds")]
    common: Option<String>,

    #[arg(short, long, help = "Generate a strong password instead")]
    generate: bool,
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

fn generate_password(length: usize) -> String {
    let mut rng = rng();

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

fn analyze(password: &str, common_file: &Option<String>) {
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
        println!(
            "{}",
            "- Include at least one special character (e.g., !@#$%^&*).".yellow()
        );
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

    match (&args.password, args.generate) {
        (Some(_), true) => {
            eprintln!(
                "{}",
                "❌ You can't provide a password and use --generate at the same time.".red()
            );
            process::exit(1);
        }
        (None, false) => {
            eprintln!(
                "{}",
                "❌ Please provide a password or use --generate.".red()
            );
            process::exit(1);
        }
        (Some(password), false) => {
            println!("Received password: {}", password);
            println!("--- Security Analysis ---");
            analyze(&password, &args.common);
        }
        (None, true) => {
            let generated = generate_password(12);
            println!("{}", "🔐 Generated secure password:".green().bold());
            println!("{}", generated.blue().bold());
            println!("--- Security Analysis ---");
            analyze(&generated, &args.common);
        }
    }
}
