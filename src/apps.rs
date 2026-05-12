use crate::tools::{ask, dnf::Dnf, flatpak::Flatpak, is_exe_in_path, run_interactively};
use std::path::Path;

pub fn install() -> anyhow::Result<()> {
  install_cosmic_copr()?;
  install_firefox()?;
  install_steam()?;
  install_discord()?;
  install_vscode()?;
  Ok(())
}

fn install_cosmic_copr() -> anyhow::Result<()> {
  let repo_file =
    Path::new("/etc/yum.repos.d/_copr:copr.fedorainfracloud.org:adil192:cosmic-epoch.repo");
  if repo_file.exists() {
    println!("Skipping 'adil192/cosmic-epoch' copr: already enabled");
    return Ok(());
  }
  if !ask("Install my repo for faster COSMIC updates?", true) {
    return Ok(());
  }
  println!("Installing my repo for faster COSMIC updates...");
  run_interactively("sudo", &["dnf", "copr", "enable", "adil192/cosmic-epoch"])?;
  println!("My builds will be installed the next time you run `sudo dnf update`.");
  Ok(())
}

fn install_firefox() -> anyhow::Result<bool> {
  install_package(&Package::new("Firefox").dnf_id("firefox"))
}

fn install_steam() -> anyhow::Result<bool> {
  install_package(&Package::new("Steam").dnf_id("steam"))
}

fn install_discord() -> anyhow::Result<bool> {
  install_package(
    &Package::new("Equibop (Discord client)")
      .flatpak_id("org.equicord.equibop")
      .alternative_flatpaks(&["dev.vencord.Vesktop", "com.discordapp.Discord"]),
  )
}

fn install_vscode() -> anyhow::Result<bool> {
  install_package(
    &Package::new("Visual Studio Code")
      .dnf_id("https://code.visualstudio.com/sha/download?build=stable&os=linux-rpm-x64")
      .alternative_exes(&["code"]),
  )
}

/// Installs a package if it's not already installed.
/// Returns true if the package is (newly/already) installed, false if not installed.
fn install_package(package: &Package) -> anyhow::Result<bool> {
  if let Some(exes) = package.alternative_exes
    && exes.iter().any(|exe| is_exe_in_path(exe))
  {
    println!("Skipping '{}' since {exes:?} on PATH", package.name);
    return Ok(true);
  }
  if let Some(ids) = package.alternative_flatpaks
    && Flatpak::exists()
    && ids.iter().any(|id| Flatpak::is_installed(id))
  {
    println!(
      "Skipping '{}' since {ids:?} flatpak already installed",
      package.name
    );
    return Ok(true);
  }
  if package.dnf_id.is_some() {
    let result = install_package_with_dnf(package)?;
    if result {
      return Ok(true);
    }
  }
  if package.flatpak_id.is_some() {
    let result = install_package_with_flatpak(package)?;
    if result {
      return Ok(true);
    }
  }
  return Ok(false);
}

fn install_package_with_dnf(package: &Package) -> anyhow::Result<bool> {
  if !Dnf::exists() {
    return Ok(false);
  }
  let Some(id) = package.dnf_id else {
    panic!("No DNF package available for {}", package.name);
  };
  if Dnf::is_installed(id) {
    println!(
      "Skipping '{}' since it's already installed with dnf",
      package.name
    );
    return Ok(true);
  }
  if !ask(&format!("Install {} with dnf?", package.name), true) {
    return Ok(false);
  }
  println!("Installing {} with dnf...", package.name);
  Dnf::install(&[id])?;
  println!();
  Ok(true)
}

fn install_package_with_flatpak(package: &Package) -> anyhow::Result<bool> {
  if !Flatpak::exists() {
    return Ok(false);
  }
  let Some(id) = package.flatpak_id else {
    panic!("No flatpak available for {}", package.name);
  };
  if Flatpak::is_installed(id) {
    println!(
      "Skipping '{}' since it's already installed with flatpak",
      package.name
    );
    return Ok(true);
  }
  if !ask(&format!("Install {} with flatpak?", package.name), true) {
    return Ok(false);
  }
  println!("Installing {} with flatpak...", package.name);
  Flatpak::install(id)?;
  println!();
  Ok(true)
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

impl Package {
  pub fn new(name: &'static str) -> Self {
    Self {
      name,
      ..Package::default()
    }
  }

  fn dnf_id(mut self, id: &'static str) -> Self {
    self.dnf_id = Some(id);
    self
  }

  fn flatpak_id(mut self, id: &'static str) -> Self {
    self.flatpak_id = Some(id);
    self
  }

  fn alternative_exes(mut self, exes: &'static [&'static str]) -> Self {
    self.alternative_exes = Some(exes);
    self
  }

  fn alternative_flatpaks(mut self, flatpaks: &'static [&'static str]) -> Self {
    self.alternative_flatpaks = Some(flatpaks);
    self
  }
}
