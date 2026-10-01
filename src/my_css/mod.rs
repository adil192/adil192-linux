pub mod firefox;
pub mod github_desktop;
pub mod qt;
pub mod steam;
pub mod thunderbird;
mod tint_css;
pub mod tokens;

use anyhow::Result;
use cached::once;

pub struct MyCss;
impl MyCss {
  /// Returns true when we're currently running in the COSMIC DE
  /// and the COSMIC `apply_theme_global` setting is true.
  pub fn enabled() -> Result<bool> {
    _enabled()
  }
}

/// Returns true.
/// (No conditions are currently used.)
#[once()]
fn _enabled() -> Result<bool> {
  Ok(true)
}
