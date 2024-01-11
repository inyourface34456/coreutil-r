use clap::Parser;
use common::*;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::MetadataExt;


#[derive(Clone)]
enum RemoveMethod {
    UnLlink,
    Wipe,
    WipeSync,
    None,
}

impl From<String> for RemoveMethod {
    fn from(value: String) -> RemoveMethod {
        return match value.as_str() {
            "unlink" => RemoveMethod::UnLlink,
            "wipe" => RemoveMethod::Wipe,
            "wipesync" => RemoveMethod::WipeSync,
            _ => RemoveMethod::None,
        };
    }
}

impl From<RemoveMethod> for clap::builder::OsStr {
    fn from(value: RemoveMethod) -> clap::builder::OsStr {
        return match value {
        RemoveMethod::UnLlink=> clap::builder::OsStr::from("warn"),
        RemoveMethod::Wipe=> clap::builder::OsStr::from("warn-nopipe"),
        RemoveMethod::WipeSync => clap::builder::OsStr::from("exit"),
        RemoveMethod::None => safly_exit!("shred: invalid argument for '--remove'\nValid arguments are:\n  - ‘wipe’\n  - ‘unlink’\n  - ‘wipesync’")
    };
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: Option<String>,

    ///change permissions to allow writing if necessary
    #[arg(short = 'f', long = "force", default_value_t = false)]
    force: bool,

    ///overwrite N times instaed of the default 3
    #[arg(short = 'n', long = "iterations", default_value_t = 3)]
    iters: i32,

    ///get random bytes from FILE
    #[arg(long = "random_source", default_value_t = String::from("/dev/random"))]
    random_source: String,

    ///shred N bytes 
    #[arg(short = 's', long = "size", default_value_t = String::new())]
    num_bytes: String,

    ///deallocte and remove file after writing
    #[arg(short = 'u', default_value_t = false)]
    deleate_file: bool,

    ///like -u but give control on how to delate
    #[arg(long = "remove", default_value = RemoveMethod::WipeSync)]
    del_mode: RemoveMethod,

    ///show progress
    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    verbose: bool,

    ///do not round file sizes up to the next full block; this is the default for non-regular files
    #[arg(short = 'x', long = "exact", default_value_t = false)]
    exact: bool,

    ///add a final overwrite with zeros to hide shreding
    #[arg(short = 'z', long = "zero", default_value_t = false)]
    zero: bool,
}

fn get_rand_bytes(s: usize, rand_source: &String) -> Vec<u8> {
    let mut f = File::open(rand_source).unwrap();
    let mut buffer = vec![0; s];

    f.read_exact(&mut buffer).unwrap();

    buffer
}

fn to_nearest_blk_size(num: i128, blk_size: i128) -> usize {
  (((num*-1)/blk_size)*blk_size*-1+blk_size).try_into().unwrap()
}

fn make_numeric(value: String) -> usize {
    value
        .chars()
        .filter_map(|x| 
          if x.is_numeric() { 
            Some(x) 
          } else { 
            None 
          }
        )
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn num_bytes_parser(value: String) -> usize {
    let value = value.to_lowercase();

    match value.parse::<usize>() {
        Ok(dat) => dat,
        Err(_) => {
            if value.starts_with('k') {
              make_numeric(value)*1000
            } else if value.starts_with('m') {
              make_numeric(value)*10000
            } else if value.starts_with('g') {
              make_numeric(value)*100000
            } else if value.starts_with('t') {
              make_numeric(value)*1000000
            } else if value.starts_with('p') {
              make_numeric(value)*10000000
            } else if value.starts_with('e') {
              make_numeric(value)*100000000
            } else if value.starts_with('z') {
              make_numeric(value)*1000000000
            } else if value.starts_with('y') {
              make_numeric(value)*10000000000
            } else {
              make_numeric(value)
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let name = match cli.name {
      Some(dat) => dat,
      None => String::new()
    };
    let file_size: u64;
    let mut file;
    let blk_size;
    let num_bytes: usize;

    if path_exists(&name) {
        let meta = fs::metadata(&name).unwrap();
        file_size = meta.len();
        blk_size = meta.blksize();
        if cli.exact {
          num_bytes = file_size as usize;
        } else if cli.num_bytes != String::new() {
          num_bytes = num_bytes_parser(cli.num_bytes)
        } else {
          num_bytes = to_nearest_blk_size(file_size.into(), blk_size.into());
        }
    } else {
        if name == String::from("-") || name == String::new() {
            file_size = match input() {
              Some(dat) => dat.len().try_into().unwrap(),
              None => safly_exit!("shred: could not read from stdin")
            };
            num_bytes = 0;
        } else {
            safly_exit!("shred: {}: does not exist", &name)
        }
    }

    for i in 0..cli.iters {
      if name == String::from("-") || name == String::new() {  
          let data: String = get_rand_bytes(file_size.try_into().unwrap(), &cli.random_source).into_iter().map(|x| x as char).collect();
          print!("{}", data)
      } else {
        file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&name)
            .unwrap();

        file.write_all(
            get_rand_bytes(num_bytes, &cli.random_source).as_slice(),
        )
        .expect("but why");
      }

      if cli.verbose {
        println!("on the {i}th iteration");
      }
    }

    if cli.zero && !(name == String::from("-") || name == String::new()) {
        file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&name)
            .unwrap();

        file.write_all(vec![0; num_bytes].as_slice())
            .expect("but why")
    }

    if cli.deleate_file {
      match fs::remove_file(&name) {
        Ok(_) => {},
        Err(e) => {
          safly_exit!("shred: {}: {}", &name, e.to_string());
        }
      }
    }
}
