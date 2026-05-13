use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::tools::{is_exe_in_path, run_interactively, run_output};

static EXISTS: OnceLock<bool> = OnceLock::new();

static INSTALLED_PACKAGES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
fn get_installed_packages() -> &'static Mutex<HashSet<String>> {
  INSTALLED_PACKAGES.get_or_init(|| {
    let output = run_output("dnf", &["list", "--installed"]).unwrap();

    let set = output.lines().map(str::to_owned).collect();

    Mutex::new(set)
  })
}

pub struct Dnf;
impl Dnf {
  pub fn exists() -> bool {
    EXISTS.get_or_init(|| is_exe_in_path("dnf")).to_owned()
  }

  pub fn is_installed(id: &str) -> bool {
    let packages = get_installed_packages().lock().unwrap();
    packages
      .iter()
      // could be package.x86_64, package.noarch, etc.
      .any(|package| package.starts_with(&format!("{id}.")))
  }

  pub fn install(ids: &[&str]) -> anyhow::Result<()> {
    let mut args = vec!["dnf", "install"];
    args.extend(ids);
    run_interactively("sudo", &args)?;

    let mut packages = get_installed_packages().lock().unwrap();
    packages.extend(ids.iter().map(|id| format!("{id}.noarch")));

    Ok(())
  }

  pub fn update(ids: &[&str]) -> anyhow::Result<()> {
    let mut args = vec!["dnf", "update"];
    args.extend(ids);
    run_interactively("sudo", &args)?;

    Ok(())
  }

  pub fn swap(ids: &[&str]) -> anyhow::Result<()> {
    let mut args = vec!["dnf", "swap"];
    args.extend(ids);
    run_interactively("sudo", &args)?;

    let mut packages = get_installed_packages().lock().unwrap();
    packages.remove(ids[0]);
    packages.insert(ids[1].to_owned());

    Ok(())
  }
}
