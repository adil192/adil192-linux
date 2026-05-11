use crate::tools::*;
use std::path::Path;

pub fn install() {
  install_cosmic_copr();
  install_firefox();
}

fn install_cosmic_copr() {
  let repo_file =
    Path::new("/etc/yum.repos.d/_copr:copr.fedorainfracloud.org:adil192:cosmic-epoch.repo");
  if repo_file.exists() {
    return;
  }
  if !ask("Install my repo for faster COSMIC updates?", true) {
    return;
  }
  println!("Installing my repo for faster COSMIC updates...");
  run_interactively("sudo", &["dnf", "copr", "enable", "adil192/cosmic-epoch"]);
  println!("My builds will be installed the next time you run `sudo dnf update`.")
}

fn install_firefox() -> bool {
  install_package(&Package {
    name: "Firefox",
    dnf_id: Some("firefox"),
    ..Package::default()
  })
}

/// Installs a package if it's not already installed.
/// Returns true if the package is (newly/already) installed, false if not installed.
fn install_package(package: &Package) -> bool {
  if let Some(exes) = package.alternative_exes
    && exes.iter().any(|exe| is_exe_in_path(exe))
  {
    return true;
  }
  if let Some(ids) = package.alternative_flatpaks
    && *flatpak::EXISTS
    && ids.iter().any(|id| flatpak::installed(id))
  {
    return true;
  }
  if package.dnf_id.is_some() {
    if install_with_dnf(package) {
      return true;
    }
  }
  if package.flatpak_id.is_some() {
    if install_with_flatpak(package) {
      return true;
    }
  }
  return false;
}

fn install_with_dnf(package: &Package) -> bool {
  if !*dnf::EXISTS {
    return false;
  }
  let Some(id) = package.dnf_id else {
    panic!("No DNF package available for {}", package.name);
  };
  if dnf::installed(id) {
    return true;
  }
  if !ask(&format!("Install {} with dnf?", package.name), true) {
    return false;
  }
  println!("Installing {} with dnf...", package.name);
  let result = dnf::install(&[id]);
  println!();
  result.is_ok()
}

fn install_with_flatpak(package: &Package) -> bool {
  if !*flatpak::EXISTS {
    return false;
  }
  let Some(id) = package.flatpak_id else {
    panic!("No flatpak available for {}", package.name);
  };
  if flatpak::installed(id) {
    return true;
  }
  if !ask(&format!("Install {} with flatpak?", package.name), true) {
    return false;
  }
  println!("Installing {} with flatpak...", package.name);
  let result = flatpak::install(&[id]);
  println!();
  result.is_ok()
}

#[derive(Default)]
struct Package {
  /// User-facing name for this package
  name: &'static str,
  /// Dnf package name, if available
  dnf_id: Option<&'static str>,
  /// Flatpak package name, if available
  flatpak_id: Option<&'static str>,
  /// If any of these are found in PATH, skip installation
  alternative_exes: Option<&'static [&'static str]>,
  /// If any of these flatpaks are installed, skip installation
  alternative_flatpaks: Option<&'static [&'static str]>,
}
