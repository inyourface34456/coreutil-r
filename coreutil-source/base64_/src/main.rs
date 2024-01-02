use base64ct::{Base64, Encoding};
use clap::Parser;
use common::*;
use std::fs;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: Option<String>,

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

    match cli.name {
      Some(name) => {
        if !cli.decode {
            if name == "-" {
                let contents = input().unwrap();
                println!(
                    "{}",
                  Base64::encode_string(contents.as_bytes())
                );
            } else {
                if path_exists(&name) {
                    let contents = match fs::read_to_string(&name) {
                        Ok(dat) => dat,
                        Err(e) => {
                            eprintln!("base64: {}: {}", &name, e);
                            "".to_string()
                        }
                    };
                    let output = Base64::encode_string(contents.as_bytes());
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
                    eprintln!("base64: {}: file does not exist", &name);
                }
            }
        } else {
            if name == "-" {
                let contents = input().unwrap();
                let dat = match Base64::decode_vec(contents.as_ref()) {
                    Ok(thing) => thing,
                    Err(e) => {
                        eprintln!("base64: {}", e);
                        exit(1)
                    }
                };
                println!("{}", String::from_utf8(dat).unwrap());
            } else {
                if path_exists(&name) {
                    let contents = match fs::read_to_string(&name) {
                        Ok(dat) => dat,
                        Err(e) => {
                            eprintln!("base64: {}: {}", &name, e);
                            "".to_string()
                        }
                    };
                    let dat = match Base64::decode_vec(contents.as_ref()) {
                        Ok(thing) => thing,
                        Err(e) => {
                            eprintln!("base64: {}", e);
                            exit(1)
                        }
                    };
                    let output = String::from_utf8(dat).unwrap();
                    print!("{}", output);
                }
            }
        }
      }
      None => {
        if cli.decode {
          let contents = input().unwrap();
          let dat = match Base64::decode_vec(contents.as_ref()) {
              Ok(thing) => thing,
              Err(e) => {
                  eprintln!("base64: {}", e);
                  exit(1)
              }
          };
          println!("{}", String::from_utf8(dat).unwrap());
        } else {
          let contents = input().unwrap();
          println!(
              "{}",
            Base64::encode_string(contents.as_bytes())
          );
        }
      }
    }
}