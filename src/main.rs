mod apps;
mod args;
mod tools;

use clap::Parser;

fn main() -> anyhow::Result<()> {
  let args = args::Args::parse();

  if args.install_apps {
    apps::install()?;
  }

  Ok(())
}
