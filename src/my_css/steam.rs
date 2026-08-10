use std::env::var;
use std::path::{Path, PathBuf};

use anyhow::Result;
use cmd_lib::run_fun;

use crate::my_css::tint_css::untint_css;
use crate::my_css::{MyCss, tint_css};
use crate::tools::ask;

impl MyCss {
  pub fn theme_steam() -> Result<()> {
    assert!(Self::enabled()?);
    println!("Tinting Steam");

    let app = find_steamui()?;
    // Steam uses a hue of 197-231deg for neutral/primary elements.
    tint_css::tint_css(&app, 195.0, 235.0, false)?;

    Ok(())
  }

  pub fn untheme_steam() -> Result<bool> {
    if !ask("Untheme Steam?", true) {
      return Ok(false);
    }
    println!("Resetting Steam css...");

    let app = find_steamui()?;
    untint_css(&app)?;

    Ok(true)
  }
}

fn find_steamui() -> Result<PathBuf> {
  let home = var("HOME")?;
  let real_steamui = run_fun!(realpath $home/.steam/steam/steamui/css)?;
  Ok(Path::new(&real_steamui).to_owned())
}
