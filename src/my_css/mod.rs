pub mod firefox;
pub mod github_desktop;
pub mod qt;
pub mod thunderbird;
pub mod tokens;

use anyhow::Result;
use cached::once;
use cosmic::config::CosmicTk;
use cosmic::cosmic_config::CosmicConfigEntry;

pub struct MyCss;
impl MyCss {
  /// Returns true when we're currently running in the COSMIC DE
  /// and the COSMIC `apply_theme_global` setting is true.
  pub fn enabled() -> Result<bool> {
    _enabled()
  }
}

/// Returns true when we're currently running in the COSMIC DE
/// and the COSMIC `apply_theme_global` setting is true.
#[once()]
fn _enabled() -> Result<bool> {
  let tk_helper = CosmicTk::config()?;
  let tk = match CosmicTk::get_entry(&tk_helper) {
    Ok(tk) => tk,
    Err((_errs, tk)) => tk,
  };

  Ok(tk.apply_theme_global)
}
