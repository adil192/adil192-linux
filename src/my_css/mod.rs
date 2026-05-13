pub mod firefox;
pub mod thunderbird;
pub mod tokens;

use std::env::var;

use anyhow::Result;
use cached::proc_macro::once;
use cosmic::{config::CosmicTk, cosmic_config::CosmicConfigEntry};

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
#[once(result = true)]
fn _enabled() -> Result<bool> {
  if var("XDG_SESSION_DESKTOP")? != "COSMIC" {
    return Ok(false);
  }

  let tk_helper = CosmicTk::config()?;
  let tk = match CosmicTk::get_entry(&tk_helper) {
    Ok(tk) => tk,
    Err((_errs, tk)) => tk,
  };

  Ok(tk.apply_theme_global)
}
