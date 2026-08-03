use std::env::var;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

use anyhow::{Result, anyhow, bail};

use crate::tools::ask;

pub struct MyHome;
impl MyHome {
  /// Symlinks files from `my_home` to your actual home directory.
  pub fn install() -> Result<()> {
    let home = var("HOME")?;
    let pwd = var("PWD")?;
    let my_home = Path::new(&pwd).join("my_home");

    let relative_paths = [".config/mpv", ".config/zed/settings.json"];
    for relative_path in relative_paths {
      let tracked_file = my_home.join(relative_path);
      let target_path = Path::new(&home).join(relative_path);
      if target_path.is_symlink() {
        return Ok(());
      }
      if !ask(&format!("Install ~/{}?", relative_path), true) {
        return Ok(());
      }
      if target_path.exists() {
        if !ask(
          &format!("└─ Already exists, overwrite ~/{}?", relative_path),
          false,
        ) {
          return Ok(());
        }
        fs::remove_file(&target_path)?;
      }
      if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
      }
      symlink(tracked_file, target_path)?;
    }

    Ok(())
  }

  /// Tracks a local file into this git repo.
  /// The original file will be moved into `my_home` and symlinked back.
  pub fn track() -> Result<()> {
    let home = var("HOME")?;
    let pwd = var("PWD")?;

    let orig_file_str = std::env::args()
      .nth(2)
      .ok_or_else(|| anyhow!("Usage: cargo run track ~/path/to/file"))?;
    let orig_file = Path::new(&orig_file_str);

    let relative_path = orig_file.strip_prefix(&home)?;

    let tracked_file = Path::new(&pwd).join("my_home").join(relative_path);
    if let Some(parent) = tracked_file.parent() {
      fs::create_dir_all(parent)?;
    }

    fs::copy(orig_file, &tracked_file)?;
    fs::remove_file(orig_file)?;
    if symlink(&tracked_file, orig_file).is_err() {
      _ = fs::copy(&tracked_file, orig_file);
      bail!(
        "Failed to symlink {} to {}, restoring original file...",
        tracked_file.to_string_lossy(),
        orig_file.to_string_lossy(),
      );
    }

    Ok(())
  }
}
