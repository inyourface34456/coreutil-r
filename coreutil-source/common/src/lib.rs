#![allow(internal_features)]
#![feature(allow_internal_unstable)]
use clap::Parser;
use getch::Getch;
use std::fs;
use std::process::exit as exit_std;

pub fn input() -> Option<String> {
    let mut buffer = "".to_string();
    let input = Getch::new();
    let mut letter;
    loop {
        letter = input.getch().unwrap() as char;
        match letter {
            '\n' => break,
            '\x04' => break,
            _ => buffer.push(letter),
        }
    }
    if letter == '\n' {
        return Some(buffer);
    } else if letter == '\x04' {
        return None;
    } else {
        panic!();
    }
}

pub fn input_bytes() -> Vec<u8> {
    let mut buffer = Vec::new();
    let input = Getch::new();
    let mut letter;
    loop {
        letter = input.getch().unwrap() as char;
        match letter {
            '\x04' => break,
            '\n' => break,
            _ => {
                // println!("{}", letter as u8);
                buffer.push(letter as u8);
            }
        }
    }
    if letter == '\x0a' {
        return buffer;
    } else {
        panic!();
    }
}

pub fn char_to_str(subject: char) -> String {
    let mut temp: [u8; 4] = [0; 4];
    subject.encode_utf8(&mut temp);
    let temp2: &str = std::str::from_utf8(&temp).unwrap();
    temp2.to_owned()
}

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct CliOnlyOneArg {
    pub name: String,
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn exit(code: i32) -> ! {
    exit_std(code)
}

#[macro_export]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! safly_exit  {
  () => {
          std::process::exit(1);
      };
      ($($arg:tt)*) => {{
          std::io::_eprint(std::format_args_nl!($($arg)*));
          std::process::exit(1);
      }};
  }
