use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use cmd_lib::{run_cmd, run_fun};

use crate::tools::is_exe_in_path;

static EXISTS: OnceLock<bool> = OnceLock::new();

static INSTALLED_PACKAGES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
fn get_installed_packages() -> &'static Mutex<HashSet<String>> {
  INSTALLED_PACKAGES.get_or_init(|| {
    let output = run_fun!(dnf list --installed).unwrap();
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
    // Workaround https://github.com/rust-shell-script/rust_cmd_lib/issues/42
    let args: Vec<_> = ["sudo", "dnf", "install"].iter().chain(ids).collect();
    run_cmd!($[args])?;

    let mut packages = get_installed_packages().lock().unwrap();
    packages.extend(ids.iter().map(|id| format!("{id}.noarch")));

    Ok(())
  }

  pub fn update(ids: &[&str]) -> anyhow::Result<()> {
    let args: Vec<_> = ["sudo", "dnf", "update"].iter().chain(ids).collect();
    run_cmd!($[args])?;
    Ok(())
  }

  pub fn swap(ids: &[&str]) -> anyhow::Result<()> {
    let args: Vec<_> = ["sudo", "dnf", "swap"].iter().chain(ids).collect();
    run_cmd!($[args])?;

    let mut packages = get_installed_packages().lock().unwrap();
    packages.remove(ids[0]);
    packages.insert(ids[1].to_owned());

    Ok(())
  }
}
