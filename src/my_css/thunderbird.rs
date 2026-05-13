use std::env::var;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow, bail};

use crate::my_css::MyCss;

impl MyCss {
  pub fn theme_thunderbird() -> Result<()> {
    assert!(Self::enabled()?);
    println!("Installing Thunderbird theme...");

    let profile_dir = Thunderbird::default_profile_dir()?;

    /* our firefox-css is also applicable for thunderbird */
    let local_dir = Path::new(&var("PWD")?).join("assets/firefox-css/");
    for subpath in ["chrome/userChrome.css", "chrome/userContent.css"] {
      let src = local_dir.join(subpath);
      let dst = profile_dir.join(subpath);

      if dst.exists() || dst.is_symlink() {
        fs::remove_file(&dst)?;
      } else {
        fs::create_dir_all(dst.parent().unwrap())?;
      }
      symlink(&src, &dst)?;
      println!("Linked {src:?} to {dst:?}");
    }

    Ok(())
  }
}

struct Thunderbird;
impl Thunderbird {
  fn default_profile_dir() -> Result<PathBuf> {
    let home = var("HOME")?;

    let legacy_profiles_dir = Path::new(&home).join(".thunderbird");
    let xdg_profiles_dir = Path::new(&home).join(".var/app/org.mozilla.Thunderbird/.thunderbird");
    let profiles_dir = if legacy_profiles_dir.exists() {
      legacy_profiles_dir
    } else {
      xdg_profiles_dir
    };

    let installs_ini = profiles_dir.join("installs.ini");
    // Find the line with `Default=8972389472934.default-release`
    let default_profile_id = fs::read_to_string(installs_ini)?
      .lines()
      .find_map(|line| line.strip_prefix("Default="))
      .ok_or_else(|| {
        anyhow!("Could not find Thunderbird profile; open Thunderbird first and try again.")
      })?
      .to_owned();

    let default_profile_dir = profiles_dir.join(format!("{default_profile_id}/"));
    if !default_profile_dir.exists() {
      bail!("Thunderbird profile {default_profile_dir:?} doesn't exist!");
    }

    Ok(default_profile_dir)
  }
}
