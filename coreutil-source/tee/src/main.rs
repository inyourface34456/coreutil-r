use common::*;
use clap::Parser;
use std::convert::From;
use std::process::exit;

#[derive(Clone)]
enum Mode {
  Warn,
  WarnNopipe,
  Exit,
  ExitNopipe,
  None
}

fn safly_exit(msg: &str) -> ! {
  eprintln!("{}", msg);
  exit(1)
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
        Mode::None => safly_exit("tee: invalid argument for '--output-error'\nValid arguments are:\n  - ‘warn’\n  - ‘warn-nopipe’\n  - ‘exit’\n  - ‘exit-nopipe’")
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
    #[arg(short = 'p', default_value_t = )]
    nonpipe: bool,

    ///set behavior on write error.  See MODE below
    #[arg(long = "output-error", default_value = Mode::None)]
    error_mode: Mode
}

fn main() {
    let cli = Cli::parse();

    if cli.ignore_interupts {
      todo!();
    }
}
