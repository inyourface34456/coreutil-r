use base32::{decode, encode, Alphabet};
use clap::Parser;
use getch::Getch;
use std::fs;
use std::process::exit;
use common::input;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: String,

    ///decode data
    #[arg(short = 'd', long = "decode", default_value_t = false)]
    decode: bool,

    ///when decoding, ignore non-alphabet characters (unsuported)
    #[arg(short = 'i', long = "ignore-garbage", default_value_t = false)]
    ignore_garbage: bool,

    ///wrap encoded lines after COLS character (default 76). Use 0 to disable line wrapping
    #[arg(short = 'w', long = "wrap", default_value_t = 76)]
    wrap: usize,
}



fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn main() {
    let cli = Cli::parse();

    if !cli.decode {
        if cli.name == "-" {
            let contents = input().unwrap();
            print!(
                "{}",
                encode(Alphabet::RFC4648 { padding: true }, contents.as_bytes())
            );
        } else {
            if path_exists(&cli.name) {
                let contents = match fs::read_to_string(&cli.name) {
                    Ok(dat) => dat,
                    Err(e) => {
                        eprintln!("base32: {}: {}", &cli.name, e);
                        "".to_string()
                    }
                };
                let output = encode(Alphabet::RFC4648 { padding: true }, contents.as_bytes());
                if cli.wrap != 0 {
                    let output = output
                        .chars()
                        .enumerate()
                        .flat_map(|(i, c)| {
                            if i != 0 && i % cli.wrap == 0 {
                                Some('\n')
                            } else {
                                None
                            }
                            .into_iter()
                            .chain(std::iter::once(c))
                        })
                        .collect::<String>();
                    println!("{}", output);
                } else {
                    print!("{}", output);
                }
            } else {
                eprintln!("base32: {}: file does not exist", &cli.name);
            }
        }
    } else {
      if cli.name == "-" {
          let contents = input().unwrap();
          let dat = match decode(Alphabet::RFC4648 { padding: true }, contents.as_ref()) {
            Some(thing) => thing,
            None => {
              eprintln!("base32: invalid input");
              exit(1)
            },
          };
          print!(
              "{}",
              String::from_utf8(dat).unwrap()
          );
      } else {
        if path_exists(&cli.name) {
          let contents = match fs::read_to_string(&cli.name) {
              Ok(dat) => dat,
              Err(e) => {
                  eprintln!("base32: {}: {}", &cli.name, e);
                  "".to_string()
              }
          };
          let dat = match decode(Alphabet::RFC4648 { padding: true }, contents.as_ref()) {
            Some(thing) => thing,
            None => {
                eprintln!("base32: invalid input");
                exit(1)
              },
          };
          let output = String::from_utf8(dat).unwrap();
          print!("{}", output);
      }
    }
    }
}
