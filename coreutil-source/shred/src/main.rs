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

    ///shred N bytes (prefixes like K, M, G accepted)
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

fn to_nearest_blk_size(num: u64, blk_size: u64) -> usize {
  ((num*-1)/mult)*blk_size*-1+blk_size).try_into().unwrap()
}

fn main() {
    let cli = Cli::parse();
    let name = match cli.name {
      Some(dat) => dat,
      None => String::new()
    };
    let file_size;
    let mut file;
    let blk_size;

    if path_exists(&cli.name) {
        let meta = fs::metadata(&cli.name).unwrap();
        file_size = meta.len();
        blk_size = meta.blksize();
    } else {
        if cli.name == "-" || cli.name == "" {
            file_size = input().len();
        } else {
            safly_exit!("shred: {}: does not exist", &cli.name)
        }
    }

    for _ in 0..cli.iters {
      if cli.name == "-" || cli.name == "" {  
          let data: String = get_rand_bytes(, &cli.random_source).into_iter().map(|x| x as char).collect();
          print!("{}")
      } else {
        file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&cli.name)
            .unwrap();

        file.write_all(
            get_rand_bytes(to_nearest_blk_size(file_size, blk_size), &cli.random_source).as_slice(),
        )
        .expect("but why");
      }
    }

    if cli.zero && !(cli.name == "-" || cli.name == "") {
        file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&cli.name)
            .unwrap();

        file.write_all(vec![0; file_size.try_into().unwrap()].as_slice())
            .expect("but why")
    }
}
