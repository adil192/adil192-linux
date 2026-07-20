use std::env::var;
use std::path::Path;

use anyhow::Result;
use cached::once;
use cmd_lib::run_cmd;

use crate::tools::ask;
use crate::tools::dnf::Dnf;

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

    if Dnf::is_installed("yaru-theme")
      && Dnf::is_installed("crudini")
      && Dnf::is_installed("gnome-shell-extension-user-theme")
      && themescriptrunner_install_dir.exists()
    {
      return Ok(true);
    }

    if !ask("Install Yaru theme and theming extensions?", true) {
      return Ok(false);
    }

    println!("Installing yaru-theme with dnf...");
    Dnf::install(&["yaru-theme", "crudini", "gnome-shell-extension-user-theme"])?;
    println!();

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
      dconf write /org/gnome/shell/extensions/themescriptrunner/light-command "'${switch_gnome_theme_sh} light'";
      dconf write /org/gnome/shell/extensions/themescriptrunner/dark-command "'${switch_gnome_theme_sh} dark'";
    )?;
    if run_cmd!(gnome-extensions enable "themescriptrunner@adilhanney.com").is_err() {
      println!("Please relogin/reboot to activate this extension.");
    }

    Ok(true)
  }
}

#[once()]
fn _is_gnome() -> Result<bool> {
  Ok(var("XDG_SESSION_DESKTOP")? == "gnome")
}
