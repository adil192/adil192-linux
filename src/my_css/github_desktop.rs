use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use cmd_lib::run_fun;

use crate::my_css::tint_css::untint_css;
use crate::my_css::{MyCss, tint_css};
use crate::tools::ask;

/// GitHub Desktop uses a hue of 210deg for neutral elements.
/// Tint them to use the hue of the system theme.
const ORIGINAL_HUE: f32 = 210.0;

impl MyCss {
  pub fn theme_github_desktop() -> Result<()> {
    assert!(Self::enabled()?);
    println!("Tinting GitHub Desktop Plus");

    let app = find_app()?;
    tint_css::tint_css(&app, ORIGINAL_HUE, true)?;

    Ok(())
  }

  pub fn untheme_github_desktop() -> Result<bool> {
    if !ask("Untheme GitHub Desktop Plus?", true) {
      return Ok(false);
    }
    println!("Resetting GitHub Desktop Plus css...");

    let app = find_app()?;
    untint_css(&app)?;

    Ok(true)
  }
}

fn find_app() -> Result<PathBuf> {
  let bin = run_fun!(which desktop-plus)?;
  let real_bin = run_fun!(realpath $bin)?;
  Path::new(&real_bin)
    .parent()
    .map(Path::to_owned)
    .ok_or_else(|| anyhow!("Could not find parent dir of github desktop plus binary"))
}
