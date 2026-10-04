use std::env::var;
use std::fs;

use anyhow::Result;
use cmd_lib::run_cmd;

use crate::tools::dnf::Dnf;
use crate::tools::shelly::Shelly;
use crate::tools::{ask, ask_to_install};

pub struct FirefoxCacher;
impl FirefoxCacher {
  pub fn install() -> Result<bool> {
    let home = var("HOME").unwrap();
    let pwd = var("PWD").unwrap();

    let script_src = format!("{pwd}/assets/firefox_cache/cache_firefox.sh");
    let script_dst = format!("{home}/.local/bin/cache_firefox.sh");

    let desktop_src = format!("{pwd}/assets/firefox_cache/com.adilhanney.cache_firefox.desktop");
    let desktop_dst = format!("{home}/.config/autostart/com.adilhanney.cache_firefox.desktop");

    ask_to_install(
      "Firefox precacher",
      || {
        fs::exists(&script_dst).unwrap_or_default() && fs::exists(&desktop_dst).unwrap_or_default()
      },
      || {
        run_cmd!(
          install -Dm644 $script_src $script_dst;
          install -Dm755 $desktop_src $desktop_dst;
        )?;

        if home != "/home/ahann" {
          run_cmd!(sed "s|/home/ahann|$home|g" $desktop_dst)?;
        }

        install_vmtouch()?;

        Ok(())
      },
    )
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
  if Dnf::exists() {
    ask_to_install(
      "vmtouch for faster precaching (optional)",
      || Dnf::is_installed("vmtouch"),
      || Dnf::install(&["vmtouch"]),
    )
  } else if Shelly::exists() {
    ask_to_install(
      "vmtouch for faster precaching (optional)",
      || Shelly::is_installed_aur("vmtouch"),
      || Shelly::install_aur("vmtouch", "weltall"),
    )
  } else {
    print!("Consider installing vmtouch for faster precaching.");
    Ok(false)
  }
}
