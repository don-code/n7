use n7::numeronyze;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Numeronyzes input strings.")]
struct Options {
  /// Character to treat as delimiter between strings.
  #[structopt(short, long, default_value = " ")]
  delimiter: char,

  /// Strings of this length of greater will be numeronyzed.
  #[structopt(short, long, default_value = "4")]
  shortest_string: i32,

  /// Strings to numeronyze. Leave blank to read from stdin.
  #[structopt(parse(from_str))]
  input: Vec<String>
}

fn main() {
  let opts = Options::from_args();

  if opts.input.len() == 0 {
    for line in io::stdin().lock().lines() {
      println!("{}", numeronyze(opts.delimiter, opts.shortest_string, line.unwrap()));
    }
  } else {
    println!("{}", numeronyze(opts.delimiter, opts.shortest_string, opts.input.join(&opts.delimiter.to_string())));
  }
}
