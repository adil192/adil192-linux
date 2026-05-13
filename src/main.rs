mod apps;
mod args;
mod drivers;
mod firefox_cacher;
mod tools;

use clap::Parser;

use crate::{apps::MyApps, drivers::MyDrivers, firefox_cacher::FirefoxCacher};

fn main() -> anyhow::Result<()> {
  let args = args::Args::parse();

  if args.install_drivers {
    MyDrivers::install()?;
  }
  if args.install_apps {
    MyApps::install()?;
  }
  if args.install_firefox_cacher {
    FirefoxCacher::install()?;
  }

  Ok(())
}
