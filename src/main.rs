mod args;

use clap::Parser;

use crate::args::Args;

fn main() {
  let args = Args::parse();
  println!("{args:?}");
}
