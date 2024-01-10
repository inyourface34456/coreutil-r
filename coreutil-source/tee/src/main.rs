use common::*;
use clap::Parser;
use std::convert::From;
use std::fs;
use std::io::Write;
use ctrlc::*;

#[derive(Clone)]
enum Mode {
  Warn,
  WarnNopipe,
  Exit,
  ExitNopipe,
  None
}

impl From<String> for Mode {
  fn from(value: String) -> Mode {
    return match value.as_str() {
      "warn" => Mode::Warn,
      "warn-nopipe" => Mode::WarnNopipe,
      "exit" => Mode::Exit,
      "exit-nopipe" => Mode::ExitNopipe,
      _ => Mode::None
    };
  }
}

impl From<Mode> for clap::builder::OsStr {
  fn from(value: Mode) -> clap::builder::OsStr {
    return match value {
        Mode::Warn => clap::builder::OsStr::from("warn"),
        Mode::WarnNopipe => clap::builder::OsStr::from("warn-nopipe"),
        Mode::Exit => clap::builder::OsStr::from("exit"),
        Mode::ExitNopipe => clap::builder::OsStr::from("exit-nopipe"),
        Mode::None => safly_exit!("tee: invalid argument for '--output-error'\nValid arguments are:\n  - ‘warn’\n  - ‘warn-nopipe’\n  - ‘exit’\n  - ‘exit-nopipe’")
    };
  }
}

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: Vec<String>,

    ///append to the given FILEs, do not overwrite
    #[arg(short = 'a', default_value_t = false)]
    append: bool,

    ///ignore interupt signals
    #[arg(short = 'i', default_value_t = false)]
    ignore_interupts: bool,

    ///diagnose errors writing to nonpipes
    #[arg(short = 'p', default_value_t = false)]
    diagnose: bool,

    ///set behavior on write error.  See MODE below
    #[arg(long = "output-error", default_value = Mode::Exit)]
    error_mode: Mode
}

fn main() {
    let cli = Cli::parse();

    if cli.ignore_interupts {
      match set_handler(|| {}) {
        Ok(_) => {},
        Err(e) => {
          safly_exit!("failed to ignore interupts beacuse {}", e.to_string()));
        }
      }
    }

    let to_write = input_bytes();
    //println!("{:?}", to_write);

    for i in cli.name {
      if path_exists(i.as_str()) {
        if !cli.append {
          let mut file = fs::OpenOptions::new().write(true).truncate(true).open(&i).unwrap();
          match file.write_all(to_write.as_slice()) {
            Ok(_) => {},
            Err(e) => {
              if cli.diagnose {
                safly_exit!("ERROR: {i} failed to write beacuse of {}", e.to_string()).as_str();
              } else {
                match cli.error_mode {
                  Mode::Warn => {
                    eprintln!("WARN: {i} failed to write beacuse of {}", e.to_string());
                  }
                  Mode::WarnNopipe => {
                    eprintln!("WARN: {i} failed to write beacuse of {}", e.to_string());
                  }
                  Mode::Exit => {
                    safly_exit!("ERROR: {i} failed to write beacuse of {}", e.to_string());
                  }
                  Mode::ExitNopipe => {
                    safly_exit!("ERROR: {i} failed to write beacuse of {}", e.to_string());
                  }
                  Mode::None => panic!("things have gone truly wrong if you are seeing this")
                }
              }
            }
          }
        } else {
          let mut file = fs::File::options().append(true).open(&i).unwrap();
          match writeln!(&mut file, "{}", String::from_utf8(to_write.clone()).unwrap()) {
            Ok(_) => {},
            Err(e) => {
              if cli.diagnose {
                safly_exit!("ERROR: {i} failed to write beacuse of {}", e.to_string());
              } else {
                match cli.error_mode {
                  Mode::Warn => {
                    eprintln!("WARN: {i} failed to write beacuse of {}", e.to_string());
                  }
                  Mode::WarnNopipe => {
                    eprintln!("WARN: {i} failed to write beacuse of {}", e.to_string());
                  }
                  Mode::Exit => {
                    safly_exit!("ERROR: {i} failed to write beacuse of {}", e.to_string());
                  }
                  Mode::ExitNopipe => {
                    safly_exit!("ERROR: {i} failed to write beacuse of {}", e.to_string());
                  }
                  Mode::None => panic!("things have gone truly wrong if you are seeing this")
                }
              }
            }
          }
        }
      } else {
        safly_exit!("tee: {i}: file does not exist");
      }
    }

    
}
