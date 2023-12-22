use data_encoding::HEXUPPER;
use clap::Parser;
use ring::digest::{Context, Digest, SHA1_FOR_LEGACY_USE_ONLY};
use std::io::Read;
use std::fs::File;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
  name: Vec<String>,

  /// read in binary mode
  #[arg(long="binary", short='b', default_value_t = false)]
  binary: bool,

  /// read checksums from the FILEs and check them
  #[arg(long, short='c' ,default_value_t = String::new())]
  check: String,

  /// create a BSD-style checksum
  #[arg(long, default_value_t = false)]
  tag: bool,

  /// read in text mode (default)
  #[arg(long, short='t', default_value_t = true)]
  text: bool,

  /// end each output line with NUL, not newline, and disable file name escaping
  #[arg(long, short='z', default_value_t = false)]
  zero: bool,

  /// don't fail or report status for missing files
  #[arg(long="ignore-missing", default_value_t = false)]
  ignore_missing: bool,

  /// don't print OK for each successfully verified file
  #[arg(long, default_value_t = false)]
  quiet: bool,

  /// don't output anything, status code shows success
  #[arg(long, default_value_t = false)]
  status: bool,

  /// exit non-zero for improperly formatted checksum lines
  #[arg(long, default_value_t = false)]
  strict: bool,

  /// warn about improperly formatted checksum lines
  #[arg(long, short='w' ,default_value_t = false)]
  warn: bool
}

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


fn main() {
    let cli = Cli::parse();
    let mut seperator = "  ";

    if cli.binary {
      seperator = " *"
    }
  
    for i in cli.name {
      let data = match File::open(&i) {
        Ok(dat) => dat,
        Err(e) => {
          if cli.strict {
            println!("sha1sum: {}: {}", &i, e.to_string());
            exit(1)
          } else {
            println!("sha1sum: {}: {}", &i, e.to_string());
            continue;
          }
        }
      };
      let digest = match sha1_digest(data) {
          Ok(dat) => dat,
          Err(e) => {
            if cli.strict {
              println!("sha1sum: {}: {}", &i, e.to_string());
              exit(1)
            } else {
              println!("sha1sum: {}: {}", &i, e.to_string());
              continue;
            }
          }
        };
      
      println!("{}{}{}", HEXUPPER.encode(digest.as_ref()), seperator, &i);
    }
}
