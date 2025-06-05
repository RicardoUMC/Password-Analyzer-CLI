use clap::Parser;

#[derive(Parser)]
#[command(name = "passcheck")]
#[command(about = "Analize password strength", long_about = None)]
struct Args {
    password: String,
}

fn main() {
    let args = Args::parse();
    println!("Received password: {}", args.password);
}
