mod apps;
mod args;
mod tools;

use clap::Parser;

fn main() {
  let args = args::Args::parse();

  if args.install_apps {
    apps::install();
  }
}
