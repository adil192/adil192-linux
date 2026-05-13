mod apps;
mod drivers;
mod firefox_cacher;
mod tools;

use crate::{apps::MyApps, drivers::MyDrivers, firefox_cacher::FirefoxCacher};

fn main() -> anyhow::Result<()> {
  MyDrivers::install()?;
  MyApps::install()?;
  FirefoxCacher::install()?;

  Ok(())
}
