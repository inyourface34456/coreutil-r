use clap::Parser;
use common::*;
use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Write};
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.name.len() != 2 {
        eprintln!("Only two arguments are allowed");
        exit(1);
    }

    if path_exists(cli.name[0].as_str()) {
        let mut file_to_clone_to = match File::create(&cli.name[1]) {
            Ok(dat) => dat,
            Err(e) => {
                eprintln!("link: {}: {}", &cli.name[1], e);
                exit(1)
            }
        };

        let file_contents = match File::open(&cli.name[0]) {
            Ok(dat) => dat,
            Err(e) => {
                eprintln!("link: {}: {}", &cli.name[0], e.to_string());
                exit(1)
            }
        };
        let mut reader = BufReader::new(file_contents);
        let mut buffer = Vec::new();
        match reader.read_to_end(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("link: {}: {}", &cli.name[0], e.to_string());
                exit(1)
            }
        }

        match file_to_clone_to.write_all(buffer.as_ref()) {
            Ok(_) => exit(0),
            Err(e) => {
                eprintln!("link: {}: {}", &cli.name[1], e.to_string());
                exit(1)
            }
        }
    }
}
