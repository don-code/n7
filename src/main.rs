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

fn numeronyze_line(delimiter: char, shortest_string: i32, string: String) -> String {
  let mut output : Vec<String> = Vec::new();

  // input string may not already be split according to grammar rules

  for token in string.split(delimiter).collect::<Vec<&str>>() {
    if token.len() > (shortest_string as usize) {
      let mut output_token = String::new();
      let mut chars = token.chars();

      output_token.push(chars.next().unwrap());
      output_token.push_str(&(token.len() - 2).to_string());
      output_token.push(chars.last().unwrap());

      output.push(output_token);
    } else {
      output.push(token.to_string());
    }

  }

  return output.join(&delimiter.to_string());
}

fn main() {
  let opts = Options::from_args();

  if opts.input.len() == 0 {
    for line in io::stdin().lock().lines() {
      println!("{}", numeronyze_line(opts.delimiter, opts.shortest_string, line.unwrap()));
    }
  } else {
    println!("{}", numeronyze_line(opts.delimiter, opts.shortest_string, opts.input.join(&opts.delimiter.to_string())));
  }
}
