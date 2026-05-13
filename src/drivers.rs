use std::path::Path;

use anyhow::Result;

use crate::tools::{ask, device::Device, dnf::Dnf, dnf_repos::DnfRepos, run_interactively};

pub fn install() -> Result<()> {
  if !Dnf::exists() {
    println!("DNF is not available, skipping drivers installation.");
    return Ok(());
  }

  DnfRepos::add_rpmfusion_repos()?;
  DnfRepos::add_terra_repos()?;
  DnfRepos::add_ultramarine_repos()?;

  install_full_ffmpeg()?;
  install_gstreamer_plugins()?;
  add_mesa_copr()?;

  Ok(())
}

fn install_full_ffmpeg() -> Result<bool> {
  if !Dnf::is_installed("ffmpeg-free") {
    println!("Skipping full-fat ffmpeg: already installed");
    return Ok(true);
  }
  if !ask("Install full-fat ffmpeg?", true) {
    return Ok(false);
  }
  println!("Installing full-fat ffmpeg...");
  Dnf::swap(&["ffmpeg-free", "ffmpeg", "--allowerasing"])?;
  Dnf::install(&["libavcodec-freeworld"])?;
  Ok(true)
}

fn install_gstreamer_plugins() -> Result<bool> {
  if Dnf::is_installed("gstreamer1-plugins-ugly") {
    println!("Skipping gstreamer plugins: already installed");
    return Ok(true);
  }
  if !ask("Install gstreamer plugins?", true) {
    return Ok(false);
  }
  println!("Installing gstreamer plugins...");
  Dnf::update(&[
    "@multimedia",
    "--setopt=install_weak_deps=False",
    "--exclude=PackageKit-gstreamer-plugin",
  ])?;
  Ok(true)
}

fn add_mesa_copr() -> Result<bool> {
  let repo_file =
    Path::new("/etc/yum.repos.d/_copr:copr.fedorainfracloud.org:adil192:mesa-rc.repo");
  if repo_file.exists() {
    println!("Skipping 'adil192/mesa-rc' copr: already added");
    return Ok(true);
  }
  if !ask(
    "Add my repo for faster Mesa (graphics driver) updates?",
    true,
  ) {
    return Ok(false);
  }
  println!("Adding my repo for faster Mesa (graphics driver) updates...");
  run_interactively("sudo", &["dnf", "copr", "enable", "adil192/mesa-rc"])?;
  println!("My builds will be installed the next time you run `sudo dnf update`.");
  Ok(true)
}
