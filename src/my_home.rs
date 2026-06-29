use std::env::var;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

use anyhow::{Result, anyhow};

pub struct MyHome;
impl MyHome {
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
    symlink(&tracked_file, orig_file)?;

    Ok(())
  }
}
