use std::env::var;
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use cached::once;

use crate::tools::dnf::Dnf;
use crate::tools::{ask, run_interactively};

pub struct Yaru;
impl Yaru {
  pub fn install() -> Result<bool> {
    if !_is_gnome()? {
      return Ok(false);
    }
    if !Dnf::exists() {
      return Ok(false);
    }

    let themescriptrunner_install_dir =
      Path::new("/home/ahann/.local/share/gnome-shell/extensions/themescriptrunner@adilhanney.com");

    if Dnf::is_installed("yaru-theme") && themescriptrunner_install_dir.exists() {
      return Ok(true);
    }

    if !ask("Install Yaru theme and theming extensions?", true) {
      return Ok(false);
    }

    println!("Installing yaru-theme with dnf...");
    Dnf::install(&["yaru-theme", "gnome-shell-extension-user-theme"])?;
    println!();

    if !themescriptrunner_install_dir.exists() {
      println!("Installing themescriptrunner extension...");
      let home = var("HOME")?;
      let repo = format!("{home}/Documents/GitHub/themescriptrunner");
      if !Path::new(&repo).exists() {
        run_interactively(
          "git",
          &[
            "clone",
            "https://github.com/adil192/themescriptrunner.git",
            &repo,
          ],
        )?;
      }
      Command::new("make")
        .current_dir(repo)
        .args(["clean", "install"])
        .status()
        .expect("Failed to install themescriptrunner");
    }

    let pwd = var("PWD")?;
    let switch_gnome_theme_sh = format_args!("{pwd}/scripts/switch_gnome_theme.sh");
    run_interactively(
      "dconf",
      &[
        "write",
        "/org/gnome/shell/extensions/themescriptrunner/light-command",
        &format!("'{switch_gnome_theme_sh} light'"),
      ],
    )?;
    run_interactively(
      "dconf",
      &[
        "write",
        "/org/gnome/shell/extensions/themescriptrunner/dark-command",
        &format!("'{switch_gnome_theme_sh} dark'"),
      ],
    )?;
    if run_interactively(
      "gnome-extensions",
      &["enable", "themescriptrunner@adilhanney.com"],
    )
    .is_err()
    {
      println!("Please relogin/reboot to activate this extension.");
    }

    Ok(true)
  }
}

#[once()]
fn _is_gnome() -> Result<bool> {
  Ok(var("XDG_SESSION_DESKTOP")? == "gnome")
}
