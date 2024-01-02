use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    string: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    match cli.string {
        Some(dat) => loop {
            println!("{dat}")
        },
        None => loop {
            println!("yes")
        },
    }
}
use common::*;
