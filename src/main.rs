use clap::Parser;
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
    password.len() >= 8
}

fn main() {
    let args = Args::parse();
    let password = &args.password;
    println!("Received password: {}", password);

    println!("--- Security Analysis ---");

    println!("✔  Valid length (>=8): {}", valid_length(password));
    println!("✔  Has uppercase: {}", has_upper(password));
    println!("✔  Has lowercase: {}", has_lower(password));
    println!("✔  Has numbers: {}", has_number(password));
    println!("✔  Has symbols: {}", has_symbol(password));
}
