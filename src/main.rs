mod apps;
mod drivers;
mod firefox_cacher;
mod my_css;
mod tools;

use anyhow::bail;

use crate::apps::MyApps;
use crate::drivers::MyDrivers;
use crate::firefox_cacher::FirefoxCacher;
use crate::my_css::MyCss;

fn main() -> anyhow::Result<()> {
  let arg = match std::env::args().nth(1) {
    Some(arg) => arg,
    None => "--install".to_owned(),
  };

  match arg.as_str() {
    "--install" => install(),
    "--uninstall" => uninstall(),
    _ => bail!("Unrecognized argument: {arg}"),
  }
}

fn install() -> anyhow::Result<()> {
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

fn uninstall() -> anyhow::Result<()> {
  FirefoxCacher::uninstall()?;
  MyCss::untheme_firefox()?;
  MyCss::untheme_thunderbird()?;
  Ok(())
}
