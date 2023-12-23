use clap::Parser;
use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA1_FOR_LEGACY_USE_ONLY};
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: Vec<String>,

    // implemented
    /// read in binary mode
    #[arg(long = "binary", short = 'b', default_value_t = false)]
    binary: bool,

    // implemented
    /// read checksums from the FILEs and check them
    #[arg(long, short='c' ,default_value_t = String::new())]
    check: String,

    /// create a BSD-style checksum
    #[arg(long, default_value_t = false)]
    tag: bool,

    // implemented
    /// read in text mode (default)
    #[arg(long, short = 't', default_value_t = true)]
    text: bool,

    // implemented
    /// end each output line with NUL, not newline, and disable file name escaping
    #[arg(long, short = 'z', default_value_t = false)]
    zero: bool,

    // implemented
    /// don't fail or report status for missing files
    #[arg(long = "ignore-missing", default_value_t = false)]
    ignore_missing: bool,

    // implemented
    /// don't print OK for each successfully verified file
    #[arg(long, default_value_t = false)]
    quiet: bool,

    // implemented
    /// don't output anything, status code shows success
    #[arg(long, default_value_t = false)]
    status: bool,

    /// exit non-zero for improperly formatted checksum lines
    #[arg(long, default_value_t = false)]
    strict: bool,

    /// warn about improperly formatted checksum lines
    #[arg(long, short = 'w', default_value_t = false)]
    warn: bool,
}

static BIN_NAME: &str = "sha1sum";
static HASH_LENGTH: usize = 40;

fn sha1_digest<R: Read>(mut reader: R) -> Result<Digest, std::io::Error> {
    let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;

        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn output_hashes(cli: Cli) {
    let mut seperator = "  ";

    if cli.binary {
        seperator = " *"
    }

    for i in cli.name {
        let data = match File::open(&i) {
            Ok(dat) => dat,
            Err(e) => {
                if cli.strict {
                    println!("{BIN_NAME}: {}: {}", &i, e.to_string());
                    exit(1)
                } else {
                    println!("{BIN_NAME}: {}: {}", &i, e.to_string());
                    continue;
                }
            }
        };
        let digest = match sha1_digest(data) {
            Ok(dat) => dat,
            Err(e) => {
                if cli.strict {
                    println!("{BIN_NAME}: {}: {}", &i, e.to_string());
                    exit(1)
                } else {
                    println!("{BIN_NAME}: {}: {}", &i, e.to_string());
                    continue;
                }
            }
        };

        if cli.zero && !cli.status {
          print!("{}{}{}\x00", HEXLOWER.encode(digest.as_ref()), seperator, &i);
        } else if !cli.zero && !cli.status{
          println!("{}{}{}", HEXLOWER.encode(digest.as_ref()), seperator, &i);
        } else if cli.status {
          continue;
        }
      }
}

fn get_hashes(data: &String) -> Vec<String> {
  data
  .split('\n')
  .map(|line| {
      let line: Vec<String> =
          line.to_string().split(' ').map(|x| x.to_string()).collect();
      if line[0].len() == HASH_LENGTH {
          return line[0].to_string();
      } else {
          return "".to_string();
      }
  })
  .collect()
}

fn get_file_locs(data: &String) -> Vec<String> {
  data
  .split('\n')
  .map(|line| {
      if line.is_empty() {
          return "".to_string();
      }

      let line: Vec<String> =
          line.to_string().split(' ').map(|x| x.to_string()).collect();

      if line[0] == format!("{BIN_NAME}:").as_ref() {
        return "".to_string();
      } else if line[1].is_empty() {
          return line[2].clone();
      } else {
          let loc = line[1].clone();
          return loc.replace('*', "");
      }
  })
  .collect()
}

fn check_hashes(cli: Cli) {
  let mut failed_count = 0;
  let mut sums_to_check = File::open(&cli.check)
      .expect(format!("{BIN_NAME}: {}: could not open file", &cli.check).as_ref());
  let mut data = String::new();
  let _ = sums_to_check
      .read_to_string(&mut data)
      .expect(format!("{BIN_NAME}: {}: could not read file", &cli.check).as_ref());
  let hashes: Vec<String> = get_hashes(&data);

  let file_locs: Vec<String> = get_file_locs(&data);

  for i in 0..hashes.len()-1 {
    if &hashes[i] == &file_locs[i] {
      continue;
    } else {
      let file = File::open(&file_locs[i]).expect("could not open file");
      let digest = match sha1_digest(file) {
          Ok(dat) => dat,
          Err(e) => {
              if cli.strict {
                  println!("{BIN_NAME}: {}: {}", &i, e.to_string());
                  exit(1)
              } else {
                  println!("{BIN_NAME}: {}: {}", &i, e.to_string());
                  continue;
              }
          }
      };
      if &HEXLOWER.encode(digest.as_ref()) == &hashes[i] {
        println!("{}: OK", &file_locs[i]);
      } else {
        if !cli.ignore_missing {
          failed_count += 1;
          println!("{}: FAILED", &file_locs[i]);
        } else {
          continue;
        }
      }
    }
  }
  if failed_count > 0 && !cli.ignore_missing && !cli.status {
  println!("Warning: {} file(s) have failed", failed_count);
  } else if cli.status && failed_count > 0 {
  exit(1)
  } else if cli.status && failed_count == 0 {
  exit(0)
  }
}

fn main() {
    let cli = Cli::parse();

    if cli.check.is_empty() {
        output_hashes(cli)
    } else {
        check_hashes(cli)
    }
}
