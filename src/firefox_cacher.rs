use std::env::var;
use std::fs;

use anyhow::Result;

use crate::tools::dnf::Dnf;
use crate::tools::{ask, run_interactively};

pub struct FirefoxCacher;
impl FirefoxCacher {
  pub fn install() -> Result<bool> {
    let home = var("HOME").unwrap();
    let pwd = var("PWD").unwrap();

    let script_src = format!("{pwd}/assets/firefox_cache/cache_firefox.sh");
    let script_dst = format!("{home}/.local/bin/cache_firefox.sh");

    let desktop_src = format!("{pwd}/assets/firefox_cache/com.adilhanney.cache_firefox.desktop");
    let desktop_dst = format!("{home}/.config/autostart/com.adilhanney.cache_firefox.desktop");

    if fs::exists(&script_dst).unwrap_or_default() && fs::exists(&desktop_dst).unwrap_or_default() {
      println!("Skipping Firefox cacher: already installed");
      return Ok(true);
    }

    if !ask("Precache Firefox data on boot?", true) {
      return Ok(false);
    }
    println!("Installing Firefox precacher...");

    run_interactively("install", &["-Dm644", &script_src, &script_dst])?;
    run_interactively("install", &["-Dm755", &desktop_src, &desktop_dst])?;

    if home != "/home/ahann" {
      run_interactively("sed", &[&format!("s|/home/ahann|{home}|g"), &desktop_dst])?;
    }

    install_vmtouch()?;

    Ok(true)
  }

  pub fn uninstall() -> Result<bool> {
    if !ask("Remove Firefox precacher?", true) {
      return Ok(false);
    }
    println!("Removing Firefox precacher...");
    let home = var("HOME").unwrap();
    let script_dst = format!("{home}/.local/bin/cache_firefox.sh");
    let desktop_dst = format!("{home}/.config/autostart/com.adilhanney.cache_firefox.desktop");
    if fs::exists(&script_dst)? {
      fs::remove_file(&script_dst)?;
    }
    if fs::exists(&desktop_dst)? {
      fs::remove_file(&desktop_dst)?;
    }
    Ok(true)
  }
}

fn install_vmtouch() -> Result<bool> {
  if !Dnf::exists() {
    return Ok(false);
  }
  if Dnf::is_installed("vmtouch") {
    println!("Skipping vmtouch: already installed");
    return Ok(true);
  }
  if !ask("Install vmtouch for faster precaching (optional)?", true) {
    return Ok(false);
  }
  println!("Installing vmtouch...");
  Dnf::install(&["vmtouch"])?;
  println!();
  Ok(true)
}
