use clap::Parser;
use colored::*;
use std::process;

mod cli;
mod common;
mod password;
mod utils;

use crate::cli::Args;
use crate::password::*;

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
            let generated = generate_password(DEFAULT_LENGTH);
            println!("{}", "🔐 Generated secure password:".green().bold());
            println!("{}", generated.blue().bold());
            println!("--- Security Analysis ---");
            analyze(&generated, &args.common);
        }
    }
}
