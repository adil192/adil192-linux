use std::{
  collections::HashSet,
  sync::{Mutex, OnceLock},
};

use crate::tools::{is_exe_in_path, run_interactively, run_output};

static EXISTS: OnceLock<bool> = OnceLock::new();

static INSTALLED_FLATPAKS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
fn get_installed_flatpaks() -> &'static Mutex<HashSet<String>> {
  INSTALLED_FLATPAKS.get_or_init(|| {
    let output = run_output("flatpak", &["list", "--columns=application"]).unwrap();

    let set = output.lines().map(str::to_owned).collect();

    Mutex::new(set)
  })
}

pub struct Flatpak;
impl Flatpak {
  pub fn exists() -> bool {
    EXISTS.get_or_init(|| is_exe_in_path("flatpak")).to_owned()
  }

  pub fn is_installed(id: &str) -> bool {
    let installed = get_installed_flatpaks().lock().unwrap();
    installed.contains(id)
  }

  pub fn install(id: &str) -> anyhow::Result<()> {
    run_interactively("flatpak", &["install", id])?;

    let mut flatpaks = get_installed_flatpaks().lock().unwrap();
    flatpaks.insert(id.to_owned());

    Ok(())
  }
}
