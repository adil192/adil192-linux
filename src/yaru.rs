use std::env::var;
use std::path::Path;

use anyhow::{Result, bail};
use cmd_lib::run_cmd;

use crate::tools::ask;
use crate::tools::dnf::Dnf;
use crate::tools::shelly::Shelly;

pub struct Yaru;
impl Yaru {
  pub fn install() -> Result<bool> {
    if var("XDG_SESSION_DESKTOP").unwrap_or_default() != "gnome" {
      return Ok(false);
    }

    let themescriptrunner_install_dir =
      Path::new("/home/ahann/.local/share/gnome-shell/extensions/themescriptrunner@adilhanney.com");

    if themescriptrunner_install_dir.exists() && are_dependencies_installed() {
      return Ok(true);
    }

    if !ask("Install Yaru theme and theming extensions?", true) {
      return Ok(false);
    }

    install_dependencies()?;

    if !themescriptrunner_install_dir.exists() {
      println!("Installing themescriptrunner extension...");
      let home = var("HOME")?;
      let repo = format!("{home}/Documents/GitHub/themescriptrunner");
      if !Path::new(&repo).exists() {
        run_cmd!(git clone "https://github.com/adil192/themescriptrunner.git" ${repo})?;
      }
      run_cmd!(
        cd ${repo};
        make clean install;
      )?;
    }

    let pwd = var("PWD")?;
    let switch_gnome_theme_sh = format_args!("{pwd}/scripts/switch_gnome_theme.sh");
    run_cmd!(
      dconf write /org/gnome/shell/extensions/themescriptrunner/light-command "'/usr/bin/chrt -i 0 ${switch_gnome_theme_sh} light'";
      dconf write /org/gnome/shell/extensions/themescriptrunner/dark-command "'/usr/bin/chrt -i 0 ${switch_gnome_theme_sh} dark'";
    )?;
    if run_cmd!(gnome-extensions enable "themescriptrunner@adilhanney.com").is_err() {
      println!("Please relogin/reboot to activate this extension.");
    }

    Ok(true)
  }
}

fn are_dependencies_installed() -> bool {
  if Dnf::exists() {
    Dnf::is_installed("yaru-theme")
      && Dnf::is_installed("crudini")
      && Dnf::is_installed("gnome-shell-extension-user-theme")
  } else if Shelly::exists() {
    Shelly::is_installed_aur("yaru-sound-theme")
      && Shelly::is_installed_aur("yaru-gtk-theme")
      && Shelly::is_installed_aur("yaru-gtksourceview-theme")
      && Shelly::is_installed_aur("yaru-gnome-shell-theme")
      && Shelly::is_installed_aur("yaru-metacity-theme")
      && Shelly::is_installed_aur("yaru-icon-theme")
      && Shelly::is_installed_aur("crudini")
      && Shelly::is_installed_standard("gnome-shell-extensions")
  } else {
    false
  }
}

fn install_dependencies() -> anyhow::Result<()> {
  println!("Installing yaru theme and dependencies...");
  if Dnf::exists() {
    Dnf::install(&[
      "yaru-theme",
      "crudini",
      "gnome-shell-extension-user-theme",
      "chrt",
    ])?;

    println!("Installing Darkly Qt theme...");
    run_cmd!(
      sudo dnf copr enable deltacopy/darkly;
      sudo dnf install --disablerepo=terra darkly;
    )?;

    println!();
    Ok(())
  } else if Shelly::exists() {
    run_cmd!(shelly install aur --needed yaru-sound-theme yaru-gtk-theme yaru-gtksourceview-theme yaru-gnome-shell-theme yaru-metacity-theme yaru-icon-theme)?;
    Shelly::install_aur("crudini", "LukeShortCloud")?;
    Shelly::install_standard(&["gnome-shell-extensions"])?;

    println!("Installing Darkly Qt theme...");
    Shelly::install_aur("darkly", "DeltaCopy")?;

    println!();
    Ok(())
  } else {
    bail!("No package manager found.")
  }
}
