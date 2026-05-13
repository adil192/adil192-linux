mod apps;
mod drivers;
mod firefox_cacher;
mod my_css;
mod tools;

use crate::apps::MyApps;
use crate::drivers::MyDrivers;
use crate::firefox_cacher::FirefoxCacher;
use crate::my_css::MyCss;

fn main() -> anyhow::Result<()> {
  MyDrivers::install()?;
  MyApps::install()?;
  FirefoxCacher::install()?;

  if MyCss::enabled()? {
    MyCss::generate_tokens()?;
    MyCss::theme_firefox()?;
    MyCss::theme_thunderbird()?;
  } else {
    println!("Skipping theme generation: not running COSMIC");
  }

  Ok(())
}
