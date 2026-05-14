use std::env::var;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use anyhow::{Ok, Result, anyhow, bail};
use serde_json::{Value, json};

use crate::my_css::MyCss;
use crate::tools::ask;

impl MyCss {
  pub fn theme_firefox() -> Result<()> {
    assert!(Self::enabled()?);
    println!("Installing Firefox theme...");

    let profile_dir = Firefox::default_profile_dir()?;
    Firefox::alter_user_js(&profile_dir)?;

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

  pub fn untheme_firefox() -> Result<bool> {
    if !ask("Uninstall Firefox theme?", true) {
      return Ok(false);
    }
    println!("Uninstalling Firefox theme...");
    let profile_dir = Firefox::default_profile_dir()?;
    for subpath in ["chrome/userChrome.css", "chrome/userContent.css"] {
      let dst = profile_dir.join(subpath);
      if dst.exists() || dst.is_symlink() {
        fs::remove_file(&dst)?;
      }
    }
    Ok(true)
  }
}

struct Firefox;
impl Firefox {
  fn default_profile_dir() -> Result<PathBuf> {
    let home = var("HOME")?;

    // Firefox 147 (Jan 26) uses the XDG base directories spec,
    // but grandfathered installs stay in `~/.mozilla/firefox`.
    let legacy_profiles_dir = Path::new(&home).join(".mozilla/firefox");
    let xdg_profiles_dir = Path::new(&home).join(".var/app/org.mozilla.firefox/.mozilla/firefox");
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
      .ok_or_else(|| anyhow!("Could not find Firefox profile; open Firefox first and try again."))?
      .to_owned();

    let default_profile_dir = profiles_dir.join(format!("{default_profile_id}/"));
    if !default_profile_dir.exists() {
      bail!("Firefox profile {default_profile_dir:?} doesn't exist!");
    }

    Ok(default_profile_dir)
  }

  fn alter_user_js(profile_dir: &Path) -> Result<()> {
    let user_js = profile_dir.join("user.js");
    if !user_js.exists() {
      bail!("Could not find user.js: open Firefox first and try again.");
    }

    let mut changes = 0;
    let mut lines: Vec<String> = fs::read_to_string(&user_js)?
      .lines()
      .map(str::to_owned)
      .collect();

    let mut insert_setting = |key: &str, value: &Value| -> Result<()> {
      let encoded_value = serde_json::to_string(&value)?;
      let new_line = format!("user_pref(\"{key}\", {encoded_value});");
      let prefix = format!("user_pref(\"{key}\",");
      for line in &mut lines {
        if line.starts_with(&prefix) {
          if line != &new_line {
            changes += 1;
            println!("  {new_line}");
            *line = new_line;
          }
          return Ok(());
        }
      }
      changes += 1;
      println!("  {new_line}");
      lines.push(new_line);
      Ok(())
    };
    let settings = json!({
      // Replace the Fedora start page with a blank page
      "browser.startup.homepage": "about:newtab",
      // Enable our userChrome.css
      "toolkit.legacyUserProfileCustomizations.stylesheets": true,
      // Enable transparency effects
      "browser.tabs.allow_transparent_browser": true,
      "widget.transparent-windows": true,
    })
    .as_object()
    .unwrap()
    .to_owned();

    for (key, value) in settings {
      insert_setting(&key, &value)?;
    }

    if changes > 0 {
      fs::write(&user_js, lines.join("\n"))?;
    }

    println!("Altered {changes} settings in user.js");
    Ok(())
  }
}
