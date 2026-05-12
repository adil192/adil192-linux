use crate::tools::{
  ask, dnf::Dnf, flatpak::Flatpak, is_exe_in_path, run_interactively, run_output,
};
use anyhow::{Ok, Result};
use regex::Regex;
use std::{env::var, fs, io::Write, path::Path, process::Command};

pub fn install() -> Result<()> {
  install_package(&Package::new("Firefox").dnf_id("firefox"))?;
  install_package(&Package::new("Steam").dnf_id("steam"))?;
  install_package(
    &Package::new("Equibop (Discord client)")
      .flatpak_id("org.equicord.equibop")
      .alternative_flatpaks(&["dev.vencord.Vesktop", "com.discordapp.Discord"]),
  )?;
  install_package(
    &Package::new("Visual Studio Code")
      .dnf_id("https://code.visualstudio.com/sha/download?build=stable&os=linux-rpm-x64")
      .alternative_exes(&["code"]),
  )?;
  install_package(
    &Package::new("Spotify")
      .flatpak_id("com.spotify.Client")
      .alternative_exes(&["spotify"]),
  )?;
  install_package(&Package::new("Ricochlime").flatpak_id("com.adilhanney.ricochlime"))?;
  install_package(&Package::new("Saber").flatpak_id("com.adilhanney.saber"))?;
  install_package(&Package::new("Prism Launcher").flatpak_id("org.prismlauncher.PrismLauncher"))?;
  install_package(
    &Package::new("Wine")
      .dnf_id("wine")
      .alternative_exes(&["wine"]),
  )?;
  install_package(
    &Package::new("Gear Lever (AppImage integration").flatpak_id("it.mijorus.gearlever"),
  )?;

  install_cosmic_copr()?;
  install_jetbrains_toolbox()?;
  install_android_emulator_integration()?;
  install_zed()?;
  install_git_credential_manager()?;
  install_github_desktop_plus()?;
  install_qt_breeze_theme()?;
  if install_package(&Package::new("VLC").dnf_id("vlc"))? {
    install_upscaled_vlc()?;
  }
  install_chromium()?;
  install_lm_studio()?;

  Ok(())
}

fn install_cosmic_copr() -> Result<()> {
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

fn install_jetbrains_toolbox() -> Result<bool> {
  let home = var("HOME")?;
  let install_dir = Path::new(&home).join(".local/share/JetBrains/Toolbox");
  let exe = install_dir.join("bin/jetbrains-toolbox");

  if exe.exists() {
    println!("Skipping Jetbrains Toolbox: already installed");
    return Ok(true);
  }
  if !ask("Install Jetbrains Toolbox?", true) {
    return Ok(false);
  }
  println!("Installing Jetbrains Toolbox...");

  let tar_file = Path::new("/tmp/jetbrains-toolbox.tar.gz");
  let download_url = "https://download.jetbrains.com/toolbox/jetbrains-toolbox-3.4.3.81140.tar.gz";
  run_interactively("wget", &[download_url, "-O", &tar_file.to_string_lossy()])?;
  fs::create_dir_all(&install_dir)?;
  run_interactively(
    "tar",
    &[
      "-xf",
      &tar_file.to_string_lossy(),
      "-C",
      &install_dir.to_string_lossy(),
      "--strip-components=1",
    ],
  )?;
  fs::remove_file(tar_file)?;
  run_output("chmod", &["+x", &exe.to_string_lossy()])?;

  // TODO(adil192): Check this continues running after adil192-linux exits
  Command::new(exe).spawn()?;

  Ok(true)
}

fn install_android_emulator_integration() -> Result<bool> {
  let home = var("HOME")?;
  let desktop_file =
    Path::new(&home).join(".local/share/applications/com.adilhanney.pixel8.desktop");
  let icon_file =
    Path::new(&home).join(".local/share/icons/hicolor/256x256/apps/com.adilhanney.pixel8.png");

  if desktop_file.exists() && icon_file.exists() {
    println!("Skipping Android Emulator integration: already installed");
    return Ok(true);
  }
  if !ask(&format!("Install Android Emulator integration?"), true) {
    return Ok(false);
  }
  println!("Installing Android Emulator integration...");

  fs::create_dir_all(desktop_file.parent().unwrap())?;
  fs::copy(
    Path::new("assets/emulator_integration/com.adilhanney.pixel8.desktop"),
    &desktop_file,
  )?;

  fs::create_dir_all(icon_file.parent().unwrap())?;
  fs::copy(
    "assets/emulator_integration/com.adilhanney.pixel8.png",
    &icon_file,
  )?;

  if home != "/home/ahann" {
    println!("Patching .desktop file to use {home} instead of /home/ahann");
    let mut content = fs::read_to_string(&desktop_file)?;
    content = content.replace("/home/ahann", &home);
    fs::write(&desktop_file, content)?;
  }

  Ok(true)
}

fn install_zed() -> Result<bool> {
  if is_exe_in_path("zed") {
    println!("Skipping Zed: already on PATH");
    return Ok(true);
  }
  if !ask("Install Zed?", true) {
    return Ok(false);
  }
  println!("Installing Zed...");
  run_interactively(
    "wget",
    &["https://zed.dev/install.sh", "-O", "/tmp/install-zed.sh"],
  )?;
  run_interactively("bash", &["/tmp/install-zed.sh"])?;
  Ok(true)
}

fn install_git_credential_manager() -> Result<bool> {
  if is_exe_in_path("git-credential-manager") {
    println!("Skipping Git Credential Manager: already on PATH");
    return Ok(true);
  }
  if !ask("Install Git Credential Manager?", true) {
    return Ok(false);
  }
  println!("Installing Git Credential Manager...");

  let releases: serde_json::Value = reqwest::blocking::get(
    "https://api.github.com/repos/git-ecosystem/git-credential-manager/releases/latest",
  )?
  .json()?;

  let asset_name_regex = Regex::new(r"^gcm-linux-x64-[0-9.]+\.tar\.gz$")?;
  let download_url = releases["assets"]
    .as_array()
    .unwrap()
    .iter()
    .find_map(|asset| {
      let name = asset["name"].as_str().unwrap();
      if asset_name_regex.is_match(name) {
        Some(asset["browser_download_url"].as_str().unwrap())
      } else {
        None
      }
    })
    .unwrap();

  let tmp_file = "/tmp/gcm-linux-x64.tar.gz";
  run_interactively("wget", &[download_url, "-O", tmp_file])?;
  run_interactively("sudo", &["tar", "-xvf", tmp_file, "-C", "/usr/local/bin"])?;
  fs::remove_file(tmp_file)?;
  run_interactively("/usr/local/bin/git-credential-manager", &["configure"])?;
  Ok(true)
}

fn install_github_desktop_plus() -> Result<bool> {
  if is_exe_in_path("github-desktop-plus") || is_exe_in_path("github-desktop") {
    println!("Skipping GitHub Desktop Plus: already installed");
    return Ok(true);
  }
  if !Dnf::exists() {
    println!("Skipping GitHub Desktop Plus: no dnf found!");
    return Ok(false);
  }
  if !ask("Install GitHub Desktop Plus?", true) {
    return Ok(false);
  }
  println!("Installing GitHub Desktop Plus...");

  run_interactively(
    "sudo",
    &["rpm", "--import", "https://gpg.polrivero.com/public.key"],
  )?;
  run_interactively(
    "bash",
    &[
      "-c",
      "echo -e '[github-desktop-plus]\nname=GitHub Desktop Plus\nbaseurl=https://rpm.github-desktop.polrivero.com/\nenabled=1\ngpgcheck=1\nrepo_gpgcheck=1\ngpgkey=https://gpg.polrivero.com/public.key' | sudo tee /etc/yum.repos.d/github-desktop-plus.repo",
    ],
  )?;
  Dnf::install(&["github-desktop-plus"])?;
  println!();
  Ok(true)
}

fn install_qt_breeze_theme() -> Result<bool> {
  if !Dnf::exists() {
    println!("Skipping Qt Breeze theme: no dnf found!");
    return Ok(false);
  }
  if Dnf::is_installed("plasma-breeze") {
    println!("Skipping Qt Breeze theme: already installed");
    return Ok(true);
  }
  if !ask("Install Qt Breeze theme?", true) {
    return Ok(false);
  }
  println!("Installing Qt Breeze theme...");
  Dnf::install(&["plasma-breeze", "breeze-icon-theme", "qt5ct", "qt6ct"])?;
  println!();
  Ok(true)
}

fn install_upscaled_vlc() -> Result<bool> {
  if is_exe_in_path("upscaled_vlc.sh") {
    println!("Skipping Upscaled VLC: already installed");
    return Ok(true);
  }
  let repo = "https://github.com/adil192/upscaled_vlc";
  if !ask(&format!("Install Upscaled VLC ({repo})?"), true) {
    return Ok(false);
  }
  println!("Installing Upscaled VLC...");

  let url = "https://raw.githubusercontent.com/adil192/upscaled_vlc/main/install.sh";
  let tmp_file = "/tmp/install_upscaled_vlc.sh";
  run_interactively("wget", &[url, "-O", tmp_file])?;
  run_interactively("bash", &[tmp_file])?;
  fs::remove_file(tmp_file)?;
  Ok(true)
}

fn install_chromium() -> Result<bool> {
  let installed = install_package(
    &Package::new("Chromium")
      .flatpak_id("org.chromium.Chromium")
      .alternative_flatpaks(&["com.google.Chrome"])
      .alternative_exes(&["chromium", "google-chrome", "chrome"]),
  )?;
  if !installed {
    return Ok(installed);
  }

  // Set CHROME_EXECUTABLE so Flutter can find the flatpak
  let chrome_executable = var("CHROME_EXECUTABLE").unwrap_or_default();
  if !chrome_executable.is_empty() {
    return Ok(true);
  }
  let home = var("HOME")?;
  let mut bashrc = fs::OpenOptions::new()
    .append(true)
    .open(format!("{home}/.bashrc"))
    .unwrap();
  writeln!(
    bashrc,
    "export CHROME_EXECUTABLE=\"{home}/.local/share/flatpak/app/org.chromium.Chromium/x86_64/stable/active/export/bin/org.chromium.Chromium\""
  )?;
  Ok(true)
}

fn install_lm_studio() -> Result<bool> {
  let home = var("HOME")?;
  let applications_dir = Path::new(&home).join("Applications");
  fs::create_dir_all(&applications_dir)?;

  let target = applications_dir.join("lmstudio.appimage");
  if target.exists() {
    println!("Skipping LM Studio: already installed");
    return Ok(true);
  }
  let name_regex = Regex::new(r"[Ll][Mm]-?[Ss]tudio.*\.[Aa]pp[Ii]mage")?;
  if fs::read_dir(&applications_dir)?
    .any(|entry| entry.is_ok_and(|entry| name_regex.is_match(&entry.file_name().to_string_lossy())))
  {
    println!("Skipping LM Studio: already installed");
    return Ok(true);
  }

  if !ask("Install LM Studio?", true) {
    return Ok(false);
  }
  println!("Installing LM Studio...");

  run_interactively(
    "wget",
    &[
      "-O",
      &target.to_string_lossy(),
      "https://lmstudio.ai/download/latest/linux/x64",
    ],
  )?;
  Ok(true)
}

/// Installs a package if it's not already installed.
/// Returns true if the package is (newly/already) installed, false if not installed.
fn install_package(package: &Package) -> Result<bool> {
  let name = package.name;
  if let Some(exes) = package.alternative_exes
    && exes.iter().any(|exe| is_exe_in_path(exe))
  {
    println!("Skipping {name}: {exes:?} on PATH");
    return Ok(true);
  }
  if let Some(ids) = package.alternative_flatpaks
    && Flatpak::exists()
    && ids.iter().any(|id| Flatpak::is_installed(id))
  {
    println!("Skipping {name}: {ids:?} flatpak already installed");
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

fn install_package_with_dnf(package: &Package) -> Result<bool> {
  if !Dnf::exists() {
    return Ok(false);
  }
  let name = package.name;
  let Some(id) = package.dnf_id else {
    panic!("No DNF package available for {name}");
  };
  if Dnf::is_installed(id) {
    println!("Skipping {name}: already installed with dnf");
    return Ok(true);
  }
  if !ask(&format!("Install {name} with dnf?"), true) {
    return Ok(false);
  }
  println!("Installing {name} with dnf...");
  Dnf::install(&[id])?;
  println!();
  Ok(true)
}

fn install_package_with_flatpak(package: &Package) -> Result<bool> {
  if !Flatpak::exists() {
    return Ok(false);
  }
  let name = package.name;
  let Some(id) = package.flatpak_id else {
    panic!("No flatpak available for {name}");
  };
  if Flatpak::is_installed(id) {
    println!("Skipping {name}: already installed with flatpak");
    return Ok(true);
  }
  if !ask(&format!("Install {name} with flatpak?"), true) {
    return Ok(false);
  }
  println!("Installing {name} with flatpak...");
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
