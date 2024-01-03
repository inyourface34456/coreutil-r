use clap::Parser;
use common::*;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: Option<String>,

    ///do not output the trealing newline
    #[arg(short = 'n', default_value_t = false)]
    newline: bool,

    ///enable interpertation of backslash escapes
    #[arg(short = 'e', default_value_t = false)]
    escapes: bool,

    ///enable interpertation of backslash escapes (does nothing)
    #[arg(short = 'E', default_value_t = true)]
    disables_escapes: bool,
}

static ESCAPES: [&str; 10] = [
    "\\\\", "\\a", "\\b", "\\c", "\\e", "\\f", "\\n", "\\r", "\\t", "\\v",
];

static CHARS: [char; 10] = ['\\', '\x07', '\x08', '\x03', '\x1b', '\x0c', '\x0a', '\x0d', '\x09', '\x0b'];

fn make_hex() -> [char; 256] {
  let mut array: [char; 256] = [0 as char; 256];
  for i in 0..=255 as u8 {
    array[i as usize] = i as char;
  }
  array
}

fn make_lit() -> [String; 256] {
  const ARRAY_REPEAT_VALUE: std::string::String = String::new();
  let mut array: [String; 256] = [ARRAY_REPEAT_VALUE; 256];
  for i in 0..=255 as u8 {
    #[allow(unused_comparisons)]
    if 0 <= i && i < 16 {
      array[i as usize] = format!("\x5cx0{:x}", i);
    } else {
      array[i as usize] = format!("\x5cx{:x}", i);
    }
  }
  array
}

fn replce_escapes(data: String) -> String {
  let mut new_data = data;
  for (i, c) in ESCAPES.iter().enumerate() {
    new_data = new_data.replace(c, char_to_str(CHARS[i]).as_str());
  }
  if new_data.contains("\\x") {
    for (i, c) in make_lit().iter().enumerate() {
      let chars_fr = make_hex();
      new_data = new_data.replace(c, char_to_str(chars_fr[i]).as_str());
    }
  }
  new_data
}

fn main() {
    let cli = Cli::parse();

    match cli.name {
        Some(mut data) => {
            if !cli.newline {
                if cli.escapes {
                  data = replce_escapes(data);
                }
                println!("{data}");
            } else {
                if cli.escapes {
                  data = replce_escapes(data);
                }
                print!("{data}");
            }
        }
        None => {
            if !cli.newline {
                print!("");
            } else {
                println!("");
            }
        }
    }
}
