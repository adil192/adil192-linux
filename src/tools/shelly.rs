use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use anyhow::bail;
use cmd_lib::{run_cmd, run_fun};
use serde_json::Value;

use crate::tools::is_exe_in_path;

pub struct Shelly;
impl Shelly {
  pub fn exists() -> bool {
    _exists()
  }

  pub fn is_installed_standard(id: &str) -> bool {
    let packages = get_installed_standard().lock().unwrap();
    packages.contains(id)
  }

  pub fn is_installed_aur(id: &str) -> bool {
    let packages = get_installed_aur().lock().unwrap();
    packages.contains(id)
  }

  pub fn install_standard(ids: &[&str]) -> anyhow::Result<()> {
    // Workaround https://github.com/rust-shell-script/rust_cmd_lib/issues/42
    let args: Vec<_> = ["shelly", "install", "standard"]
      .iter()
      .chain(ids)
      .collect();
    run_cmd!($[args])?;

    let mut packages = get_installed_standard().lock().unwrap();
    packages.extend(ids.iter().map(|id| id.to_string()));

    Ok(())
  }

  /// To prevent malicious package takeovers,
  /// the package will not be installed if the maintainer has changed.
  pub fn install_aur(id: &str, maintainer: &str) -> anyhow::Result<()> {
    let info_json = run_fun!(shelly search aur --info $id --json)?;
    let info: Vec<Value> = serde_json::from_str(&info_json)?;
    if info.is_empty() {
      bail!("No AUR package found for {id}");
    }
    let Some(actual_maintainer) = info[0]["Maintainer"].as_str() else {
      bail!("AUR maintainer mismatch for {id}: expected {maintainer}, got None.");
    };
    if maintainer != actual_maintainer {
      bail!("AUR maintainer mismatch for {id}: expected {maintainer}, got {actual_maintainer}.");
    }

    run_cmd!(shelly install aur $id)?;

    let mut packages = get_installed_aur().lock().unwrap();
    packages.insert(id.to_owned());

    Ok(())
  }
}

#[cached::once()]
fn _exists() -> bool {
  is_exe_in_path("shelly")
}

static INSTALLED_STANDARD: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
fn get_installed_standard() -> &'static Mutex<HashSet<String>> {
  INSTALLED_STANDARD.get_or_init(|| {
    let output = run_fun!(shelly list standard --json).unwrap();
    let packages: Vec<Value> = serde_json::from_str(&output).unwrap();
    let set = packages
      .into_iter()
      .map(|value| value["Name"].as_str().unwrap().to_owned())
      .collect();
    Mutex::new(set)
  })
}

static INSTALLED_AUR: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
fn get_installed_aur() -> &'static Mutex<HashSet<String>> {
  INSTALLED_AUR.get_or_init(|| {
    let output = run_fun!(shelly list aur --json).unwrap();
    let packages: Vec<Value> = serde_json::from_str(&output).unwrap();
    let set = packages
      .into_iter()
      .map(|value| value["Name"].as_str().unwrap().to_owned())
      .collect();
    Mutex::new(set)
  })
}
