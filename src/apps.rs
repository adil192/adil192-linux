use crate::tools::dnf::Dnf;
use crate::tools::dnf_repos::DnfRepos;
use crate::tools::flatpak::Flatpak;
use crate::tools::shelly::Shelly;
use crate::tools::{ask, find_exe_in_path, is_exe_in_path};
use anyhow::{Ok, Result};
use cmd_lib::run_cmd;
use regex::Regex;
use std::env::var;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub struct MyApps;
impl MyApps {
  pub fn install() -> Result<()> {
    if Dnf::exists() {
      DnfRepos::add_rpmfusion_repos()?;
    }
    install_package(
      &Package::new("Firefox")
        .dnf_id("firefox")
        .pacman_id("firefox"),
    )?;
    install_package(&Package::new("Steam").dnf_id("steam").pacman_id("steam"))?;
    install_package(
      &Package::new("Equibop (Discord client)")
        .flatpak_id("org.equicord.equibop")
        .alternative_flatpaks(&["dev.vencord.Vesktop", "com.discordapp.Discord"])
        .alternative_exes(&["equibop"]),
    )?;
    install_package(
      &Package::new("Visual Studio Code")
        .aur_package("visual-studio-code-bin", "dcelasun")
        .dnf_id("https://code.visualstudio.com/sha/download?build=stable&os=linux-rpm-x64")
        .alternative_exes(&["code"]),
    )?;
    install_package(
      &Package::new("Spotify")
        .aur_package("spotify", "gromit")
        .flatpak_id("com.spotify.Client")
        .alternative_exes(&["spotify", "spotifast"]),
    )?;
    install_package(&Package::new("Ricochlime").flatpak_id("com.adilhanney.ricochlime"))?;
    install_package(&Package::new("Saber").flatpak_id("com.adilhanney.saber"))?;
    install_package(
      &Package::new("Prism Launcher")
        .pacman_id("prismlauncher")
        .flatpak_id("org.prismlauncher.PrismLauncher"),
    )?;
    install_package(
      &Package::new("Wine")
        .dnf_id("wine")
        .pacman_id("wine")
        .alternative_exes(&["wine"]),
    )?;
    // TODO: Install AppManager

    add_cosmic_copr()?;
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
}

fn add_cosmic_copr() -> Result<()> {
  if !Dnf::exists() {
    return Ok(());
  }
  let repo_file =
    Path::new("/etc/yum.repos.d/_copr:copr.fedorainfracloud.org:adil192:cosmic-epoch.repo");
  if repo_file.exists() {
    println!("Skipping 'adil192/cosmic-epoch' copr: already added");
    return Ok(());
  }
  if !ask("Add my repo for faster COSMIC updates?", true) {
    return Ok(());
  }
  println!("Adding my repo for faster COSMIC updates...");
  run_cmd!(sudo dnf copr enable adil192/cosmic-epoch)?;
  println!("My builds will be installed the next time you run `sudo dnf update`.");
  Ok(())
}

fn install_jetbrains_toolbox() -> Result<bool> {
  if Shelly::exists() {
    return install_package_with_aur(
      &Package::new("Jetbrains Toolbox").aur_package("jetbrains-toolbox", "freswa"),
    );
  }

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
  run_cmd!(
    wget $download_url -O $tar_file;
    mkdir -p $install_dir;
    tar -xf $tar_file -C $install_dir --strip-components=1;
    rm $tar_file;
    chmod +x $exe;
  )?;

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
  if !ask("Install Android Emulator integration?", true) {
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
    println!("Skipping Zed: already in PATH");
    return Ok(true);
  }
  if Shelly::exists() {
    return install_package_with_pacman(&Package::new("Zed").pacman_id("zed"));
  }
  if !ask("Install Zed?", true) {
    return Ok(false);
  }
  println!("Installing Zed...");
  run_cmd!(
    wget "https://zed.dev/install.sh" -O /tmp/install-zed.sh;
    bash /tmp/install-zed.sh;
    rm /tmp/install-zed.sh;
  )?;
  Ok(true)
}

fn install_git_credential_manager() -> Result<bool> {
  if is_exe_in_path("git-credential-manager") {
    println!("Skipping Git Credential Manager: already in PATH");
    return Ok(true);
  }
  if Shelly::exists() {
    return install_package_with_aur(
      &Package::new("Git Credential Manager").aur_package("git-credential-manager-bin", "hzmi"),
    );
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
  run_cmd!(
    wget $download_url -O $tmp_file;
    sudo tar -xvf $tmp_file -C /usr/local/bin;
    rm $tmp_file;
    /usr/local/bin/git-credential-manager configure;
  )?;
  Ok(true)
}

fn install_github_desktop_plus() -> Result<bool> {
  if is_exe_in_path("desktop-plus")
    || is_exe_in_path("github-desktop-plus")
    || is_exe_in_path("github-desktop")
  {
    println!("Skipping GitHub Desktop Plus: already installed");
    return Ok(true);
  }
  if Shelly::exists() {
    install_package_with_aur(
      &Package::new("GitHub Desktop Plus").aur_package("desktop-plus-bin", "polr"),
    )
  } else if Dnf::exists() {
    if !ask("Install GitHub Desktop Plus?", true) {
      return Ok(false);
    }
    println!("Installing GitHub Desktop Plus...");

    run_cmd!(
      sudo rpm --import "https://gpg.polrivero.com/public.key";
      bash -c "echo -e '[github-desktop-plus]\nname=GitHub Desktop Plus\nbaseurl=https://rpm.github-desktop.polrivero.com/\nenabled=1\ngpgcheck=1\nrepo_gpgcheck=1\ngpgkey=https://gpg.polrivero.com/public.key' | sudo tee /etc/yum.repos.d/github-desktop-plus.repo";
    )?;
    Dnf::install(&["desktop-plus"])?;
    println!();
    Ok(true)
  } else {
    println!("Skipping GitHub Desktop Plus: no available sources!");
    Ok(false)
  }
}

fn install_qt_breeze_theme() -> Result<bool> {
  if Shelly::exists() {
    if Shelly::is_installed_standard("breeze") {
      println!("Skipping Qt Breeze theme: already installed");
      return Ok(true);
    }
    if !ask("Install Qt Breeze theme?", true) {
      return Ok(false);
    }
    println!("Installing Qt Breeze theme...");
    Shelly::install_standard(&["breeze", "breeze-cursors", "breeze-icons"])?;
    Shelly::install_aur("qt6ct-kde", "ilya-fedin")?;
    println!();
    Ok(true)
  } else if Dnf::exists() {
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
  } else {
    println!("Skipping Qt Breeze theme: no available sources!");
    Ok(false)
  }
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
  run_cmd!(
    wget $url -O $tmp_file;
    bash $tmp_file;
    rm $tmp_file;
  )?;
  Ok(true)
}

fn install_chromium() -> Result<bool> {
  let installed = install_package(
    &Package::new("Chromium")
      .pacman_id("chromium")
      .flatpak_id("org.chromium.Chromium")
      .alternative_flatpaks(&["com.google.Chrome"])
      .alternative_exes(&["chromium", "google-chrome", "chrome"]),
  )?;
  if !installed {
    return Ok(installed);
  }

  // Set CHROME_EXECUTABLE so Flutter can find the flatpak
  let mut chrome_executable = var("CHROME_EXECUTABLE").unwrap_or_default();
  if !chrome_executable.is_empty() {
    return Ok(true);
  }

  chrome_executable = find_exe_in_path("chromium")
    .or_else(|| find_exe_in_path("google-chrome"))
    .or_else(|| find_exe_in_path("chrome"))
    .or_else(|| find_exe_in_path("org.chromium.Chromium"))
    .or_else(|| find_exe_in_path("com.google.Chrome"))
    .unwrap_or_default();

  if chrome_executable.is_empty() {
    println!("Warning: Could not find chromium path");
  } else {
    println!("Setting CHROME_EXECUTABLE to {chrome_executable}");
    let home = var("HOME")?;
    let mut bashrc = fs::OpenOptions::new()
      .append(true)
      .open(format!("{home}/.bashrc"))
      .unwrap();
    writeln!(bashrc, "export CHROME_EXECUTABLE='{chrome_executable}'")?;
  }

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
  let name_regex = Regex::new(r"[Ll][Mm]-?[Ss]tudio(.*\.[Aa]pp[Ii]mage)?")?;
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

  run_cmd!(wget -O $target "https://lmstudio.ai/download/latest/linux/x64")?;
  Ok(true)
}

/// Installs a package if it's not already installed.
/// Returns true if the package is (newly/already) installed, false if not installed.
fn install_package(package: &Package) -> Result<bool> {
  let name = package.name;
  if let Some(exes) = package.alternative_exes
    && exes.iter().any(|exe| is_exe_in_path(exe))
  {
    println!("Skipping {name}: already in PATH");
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
  if package.pacman_id.is_some() {
    let result = install_package_with_pacman(package)?;
    if result {
      return Ok(true);
    }
  }
  if package.aur_package.is_some() {
    let result = install_package_with_aur(package)?;
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
  Ok(false)
}

fn install_package_with_dnf(package: &Package) -> Result<bool> {
  if !Dnf::exists() {
    return Ok(false);
  }
  let name = package.name;
  let Some(id) = package.dnf_id else {
    panic!("No DNF package specified for {name}");
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

fn install_package_with_pacman(package: &Package) -> Result<bool> {
  if !Shelly::exists() {
    return Ok(false);
  }
  let name = package.name;
  let Some(id) = package.pacman_id else {
    panic!("No pacman package specified for {name}");
  };
  if Shelly::is_installed_standard(id) {
    println!("Skipping {name}: already installed with pacman");
    return Ok(true);
  }
  if !ask(&format!("Install {name} with pacman?"), true) {
    return Ok(false);
  }
  println!("Installing {name} with pacman...");
  Shelly::install_standard(&[id])?;
  println!();
  Ok(true)
}

fn install_package_with_aur(package: &Package) -> Result<bool> {
  if !Shelly::exists() {
    return Ok(false);
  }
  let name = package.name;
  let Some(ref package) = package.aur_package else {
    panic!("No AUR package specified for {name}");
  };
  if Shelly::is_installed_aur(package.id) {
    println!("Skipping {name}: already installed from AUR");
    return Ok(true);
  }
  if !ask(&format!("Install {name} from AUR?"), true) {
    return Ok(false);
  }
  println!("Installing {name} from AUR...");
  Shelly::install_aur(package.id, package.maintainer)?;
  println!();
  Ok(true)
}

fn install_package_with_flatpak(package: &Package) -> Result<bool> {
  if !Flatpak::exists() {
    return Ok(false);
  }
  let name = package.name;
  let Some(id) = package.flatpak_id else {
    panic!("No flatpak specified for {name}");
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
  /// Pacman package name, if available
  pacman_id: Option<&'static str>,
  /// AUR package, if available
  aur_package: Option<PinnedAur>,
  /// Flatpak package name, if available
  flatpak_id: Option<&'static str>,
  /// If any of these are found in PATH, skip installation
  alternative_exes: Option<&'static [&'static str]>,
  /// If any of these flatpaks are installed, skip installation
  alternative_flatpaks: Option<&'static [&'static str]>,
}
struct PinnedAur {
  id: &'static str,
  maintainer: &'static str,
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

  fn aur_package(mut self, id: &'static str, maintainer: &'static str) -> Self {
    self.aur_package = Some(PinnedAur { id, maintainer });
    self
  }

  fn pacman_id(mut self, id: &'static str) -> Self {
    self.pacman_id = Some(id);
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
