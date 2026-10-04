use crate::tools::dnf::Dnf;
use crate::tools::dnf_repos::DnfRepos;
use crate::tools::flatpak::Flatpak;
use crate::tools::shelly::Shelly;
use crate::tools::{ask, ask_to_install, find_exe_in_path, is_exe_in_path};
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
    return install_package_with_aur("Jetbrains Toolbox", "jetbrains-toolbox", "freswa");
  }

  let home = var("HOME")?;
  let install_dir = Path::new(&home).join(".local/share/JetBrains/Toolbox");
  let exe = install_dir.join("bin/jetbrains-toolbox");

  ask_to_install(
    "Jetbrains Toolbox",
    || exe.exists(),
    || {
      let tar_file = Path::new("/tmp/jetbrains-toolbox.tar.gz");
      let download_url =
        "https://download.jetbrains.com/toolbox/jetbrains-toolbox-3.4.3.81140.tar.gz";
      run_cmd!(
        wget $download_url -O $tar_file;
        mkdir -p $install_dir;
        tar -xf $tar_file -C $install_dir --strip-components=1;
        rm $tar_file;
        chmod +x $exe;
      )?;

      // TODO(adil192): Check this continues running after adil192-linux exits
      Command::new(&exe).spawn()?;
      Ok(())
    },
  )
}

fn install_android_emulator_integration() -> Result<bool> {
  let home = var("HOME")?;
  let desktop_file =
    Path::new(&home).join(".local/share/applications/com.adilhanney.pixel8.desktop");
  let icon_file =
    Path::new(&home).join(".local/share/icons/hicolor/256x256/apps/com.adilhanney.pixel8.png");

  ask_to_install(
    "Android Emulator integration",
    || desktop_file.exists() && icon_file.exists(),
    || {
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

      Ok(())
    },
  )
}

fn install_zed() -> Result<bool> {
  if is_exe_in_path("zed") {
    println!("Skipping Zed: already in PATH");
    return Ok(true);
  }
  if Shelly::exists() {
    install_package_with_pacman("Zed", "zed")
  } else {
    ask_to_install(
      "Zed",
      || false,
      || {
        run_cmd!(
          wget "https://zed.dev/install.sh" -O /tmp/install-zed.sh;
          bash /tmp/install-zed.sh;
          rm /tmp/install-zed.sh;
        )?;
        Ok(())
      },
    )
  }
}

fn install_git_credential_manager() -> Result<bool> {
  if is_exe_in_path("git-credential-manager") {
    println!("Skipping Git Credential Manager: already in PATH");
    return Ok(true);
  }
  if Shelly::exists() {
    install_package_with_aur(
      "Git Credential Manager",
      "git-credential-manager-bin",
      "hzmi",
    )
  } else {
    ask_to_install(
      "Git Credential Manager",
      || false,
      || {
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
        Ok(())
      },
    )
  }
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
    install_package_with_aur("GitHub Desktop Plus", "desktop-plus-bin", "polr")
  } else if Dnf::exists() {
    ask_to_install(
      "GitHub Desktop Plus",
      || false,
      || {
        run_cmd!(
          sudo rpm --import "https://gpg.polrivero.com/public.key";
          bash -c "echo -e '[github-desktop-plus]\nname=GitHub Desktop Plus\nbaseurl=https://rpm.github-desktop.polrivero.com/\nenabled=1\ngpgcheck=1\nrepo_gpgcheck=1\ngpgkey=https://gpg.polrivero.com/public.key' | sudo tee /etc/yum.repos.d/github-desktop-plus.repo";
        )?;
        Dnf::install(&["desktop-plus"])
      },
    )
  } else {
    println!("Skipping GitHub Desktop Plus: no available sources!");
    Ok(false)
  }
}

fn install_qt_breeze_theme() -> Result<bool> {
  if Shelly::exists() {
    ask_to_install(
      "Qt Breeze theme",
      || Shelly::is_installed_standard("breeze"),
      || {
        Shelly::install_standard(&["breeze", "breeze-cursors", "breeze-icons"])?;
        Shelly::install_aur("qt6ct-kde", "ilya-fedin")
      },
    )
  } else if Dnf::exists() {
    ask_to_install(
      "Qt Breeze theme",
      || Dnf::is_installed("plasma-breeze"),
      || Dnf::install(&["plasma-breeze", "breeze-icon-theme", "qt5ct", "qt6ct"]),
    )
  } else {
    println!("Skipping Qt Breeze theme: no available sources!");
    Ok(false)
  }
}

fn install_upscaled_vlc() -> Result<bool> {
  ask_to_install(
    "Upscaled VLC",
    || is_exe_in_path("upscaled_vlc.sh"),
    || {
      let url = "https://raw.githubusercontent.com/adil192/upscaled_vlc/main/install.sh";
      let tmp_file = "/tmp/install_upscaled_vlc.sh";
      run_cmd!(
        wget $url -O $tmp_file;
        bash $tmp_file;
        rm $tmp_file;
      )?;
      Ok(())
    },
  )
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

  ask_to_install(
    "LM Studio",
    || {
      let name_regex = Regex::new(r"[Ll][Mm]-?[Ss]tudio(.*\.[Aa]pp[Ii]mage)?").unwrap();
      fs::read_dir(&applications_dir).is_ok_and(|mut entries| {
        entries.any(|entry| {
          entry.is_ok_and(|entry| name_regex.is_match(&entry.file_name().to_string_lossy()))
        })
      })
    },
    || {
      let target = applications_dir.join("lmstudio.appimage");
      run_cmd!(wget -O $target "https://lmstudio.ai/download/latest/linux/x64")?;
      Ok(())
    },
  )
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
  if let Some(id) = package.dnf_id {
    let result = install_package_with_dnf(name, id)?;
    if result {
      return Ok(true);
    }
  }
  if let Some(id) = package.pacman_id {
    let result = install_package_with_pacman(name, id)?;
    if result {
      return Ok(true);
    }
  }
  if let Some(ref aur) = package.aur_package {
    let result = install_package_with_aur(name, aur.id, aur.maintainer)?;
    if result {
      return Ok(true);
    }
  }
  if let Some(id) = package.flatpak_id {
    let result = install_package_with_flatpak(name, id)?;
    if result {
      return Ok(true);
    }
  }
  Ok(false)
}

fn install_package_with_dnf(name: &str, id: &str) -> Result<bool> {
  if !Dnf::exists() {
    return Ok(false);
  }
  ask_to_install(
    &format!("{name} (dnf)"),
    || Dnf::is_installed(id),
    || Dnf::install(&[id]),
  )
}

fn install_package_with_pacman(name: &str, id: &str) -> Result<bool> {
  if !Shelly::exists() {
    return Ok(false);
  }
  ask_to_install(
    &format!("{name} (pacman)"),
    || Shelly::is_installed_standard(id),
    || Shelly::install_standard(&[id]),
  )
}

fn install_package_with_aur(name: &str, id: &str, maintainer: &str) -> Result<bool> {
  if !Shelly::exists() {
    return Ok(false);
  }
  ask_to_install(
    &format!("{name} (aur:{maintainer})"),
    || Shelly::is_installed_standard(id),
    || Shelly::install_aur(id, maintainer),
  )
}

fn install_package_with_flatpak(name: &str, id: &str) -> Result<bool> {
  if !Flatpak::exists() {
    return Ok(false);
  }
  ask_to_install(
    &format!("{name} (flatpak)"),
    || Flatpak::is_installed(id),
    || Flatpak::install(id),
  )
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
