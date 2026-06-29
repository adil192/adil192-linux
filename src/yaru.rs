use std::env::var;

use anyhow::Result;
use cached::proc_macro::once;

use crate::tools::dnf::Dnf;
use crate::tools::{ask, run_interactively, run_output};

pub struct Yaru;
impl Yaru {
  pub fn install() -> Result<bool> {
    if !_is_gnome()? {
      return Ok(false);
    }
    if !Dnf::exists() {
      return Ok(false);
    }

    if Dnf::is_installed("yaru-theme") && _is_switcher_extension_installed()? {
      return Ok(true);
    }

    if !ask("Install Yaru theme and theming extensions?", true) {
      return Ok(false);
    }

    println!("Installing yaru-theme with dnf...");
    Dnf::install(&["yaru-theme", "gnome-shell-extension-user-theme"])?;
    println!();

    loop {
      println!(
        "Please install the extension from https://extensions.gnome.org/extension/2236/night-theme-switcher/"
      );
      ask("Press enter when installed... ", true);
      if _is_switcher_extension_installed()? {
        break;
      }
    }

    let pwd = var("PWD").unwrap();
    let switch_gnome_theme_sh = format_args!("{pwd}/scripts/switch_gnome_theme.sh");
    run_interactively(
      "dconf",
      &[
        "write",
        "/org/gnome/shell/extensions/nightthemeswitcher/commands/sunrise",
        &format!("'{switch_gnome_theme_sh} light'"),
      ],
    )?;
    run_interactively(
      "dconf",
      &[
        "write",
        "/org/gnome/shell/extensions/nightthemeswitcher/commands/sunset",
        &format!("'{switch_gnome_theme_sh} dark'"),
      ],
    )?;
    run_interactively(
      "dconf",
      &[
        "write",
        "/org/gnome/shell/extensions/nightthemeswitcher/commands/enabled",
        "true",
      ],
    )?;

    Ok(true)
  }
}

#[once(result = true)]
fn _is_gnome() -> Result<bool> {
  Ok(var("XDG_SESSION_DESKTOP")? == "gnome")
}

fn _is_switcher_extension_installed() -> Result<bool> {
  let info = run_output(
    "gnome-extensions",
    &["info", "nightthemeswitcher@romainvigier.fr"],
  )?;
  Ok(info.contains("Enabled: Yes"))
}
