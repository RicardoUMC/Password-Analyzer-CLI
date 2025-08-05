use colored::*;
use crate::password::*;

pub fn print_suggestions(password: &str) {
    println!("\n{}", "Suggestions to improve your password:".yellow());

    if !valid_length(password) {
        println!("{}", format!("- Make it at least {} characters long.", MIN_LENGTH).yellow());
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

pub fn print_strength_bar(score: u8) {
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

