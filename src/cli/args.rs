use clap::Parser;

#[derive(Parser)]
#[command(name = "passcheck")]
#[command(about = "Analize password strength", long_about = None)]
pub struct Args {
    #[arg(help = "Password to check", required = false)]
    pub password: Option<String>,

    #[arg(short, long, help = "Path to list (wordlist) of common passwords")]
    pub common: Option<String>,

    #[arg(short, long, help = "Generate a strong password instead")]
    pub generate: bool,
}

