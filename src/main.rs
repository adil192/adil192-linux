mod apps;
mod args;
mod drivers;
mod tools;

use clap::Parser;

use crate::{apps::MyApps, drivers::MyDrivers};

fn main() -> anyhow::Result<()> {
  let args = args::Args::parse();

  if args.install_drivers {
    MyDrivers::install()?;
  }
  if args.install_apps {
    MyApps::install()?;
  }

  Ok(())
}
