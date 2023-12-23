use std::thread::sleep;
use std::time::Duration;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: String,
}

fn main() {
  let cli = Cli::parse();

  if cli.name.ends_with('s') {
    let duration = cli.name.replace('s', "").parse().expect("not a number");
    sleep(Duration::from_secs_f64(duration))
  } else if cli.name.ends_with('m') {
    let duration = cli.name.replace('m', "").parse().expect("not a number");
    sleep(Duration::from_secs_f64(duration)*60)
  } else if cli.name.ends_with('h') {
    let duration = cli.name.replace('h', "").parse().expect("not a number");
    sleep(Duration::from_secs_f64(duration)*3600)
  } else if cli.name.ends_with('d') {
    let duration = cli.name.replace('d', "").parse().expect("not a number");
    sleep(Duration::from_secs_f64(duration)*216000)
  } else {
    let duration = cli.name.parse().expect("not a number");
    sleep(Duration::from_secs_f64(duration))
  }
}
