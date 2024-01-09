use common::*;
use clap::Parser;
use std::convert::From;

#[derive(Clone)]
struct Sequence {
  start: i128,
  end: i128,
  inc: i128
}

impl From<String> for Sequence {
  fn from(value: String) -> Sequence {
    let value: Vec<String> = value.split(' ').map(|x| x.to_string()).collect();

    if value.len() < 1 {
      safly_exit("seq: missing operand")
    } else if value.len() > 3 {
      safly_exit("seq: must have at most 3 arguments")
    } else {
      let mut nums: Vec<i128> = Vec::new();

      for i in value {
        let num = match i.parse() {
          Ok(dat) => dat,
          Err(_) => {
            safly_exit(format!("seq: arguments must be numbers").as_str())
          }
        };
        nums.push(num);
      }

      match nums.len() {
        1 => {
          return Sequence {start: 1, end: nums[0], inc: 1};
        }
        2 => {
          return Sequence {start: nums[0]-1, end: nums[1], inc: 1};
        }
        3 => {
          return Sequence {start: nums[0]-nums[1], end: nums[2], inc: nums[1]};
        }
        _ => panic!("the number of arguments has changed during exacution (not in an intended way)")
      }
    }
  }
}

impl Iterator for Sequence {
  type Item = i128;

  fn next(&mut self) -> Option<Self::Item> {
    if self.start >= self.end {
      None
    } else {
      self.start = match self.start.checked_add(self.inc) {
        Some(dat) => dat,
        None => return None,
      };
      Some(self.start)
    }
  }
}

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: Sequence,

    ///use STRING to separate numbers (default: \n)
    #[arg(short = 's', long = "seperator", default_value_t = String::from("\n"))]
    seperator: String,

    ///equlize width by padding with leading zeros
    #[arg(short = 'w', long = "equal-width", default_value_t = false)]
    equal_width: bool
}

fn main() {
    let cli = Cli::parse();
    let iter = &mut cli.name.clone();

    loop {
      let i = iter.next();
      match i {
        Some(value) => {
          if !cli.equal_width {
            print!("{value}{}", cli.seperator);
          } else {
            let end_len = &cli.name.end.to_string().len();
            let zeros = end_len - value.to_string().len();
            let mut fin = String::new();
            for _ in 0..zeros {
              fin.push('0');
            }
            print!("{fin}{value:}{}", cli.seperator);
          }
        },
        None => break
      }
    }
}
