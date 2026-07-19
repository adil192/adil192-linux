use std::collections::HashSet;
use std::env::var;
use std::path::Path;

use anyhow::Result;

use crate::tools::device::Device;
use crate::tools::dnf::Dnf;
use crate::tools::dnf_repos::DnfRepos;
use crate::tools::{ask, run_interactively, run_output};

pub struct MyDrivers;
impl MyDrivers {
  /// Loosely based on https://rpmfusion.org/Howto/Multimedia
  pub fn install() -> Result<()> {
    if !Dnf::exists() {
      println!("DNF is not available, skipping drivers installation.");
      return Ok(());
    }

    DnfRepos::add_rpmfusion_repos()?;
    DnfRepos::add_terra_repos()?;

    install_full_ffmpeg()?;
    install_gstreamer_plugins()?;
    add_mesa_copr()?;

    if Device::has_intel_gpu() {
      install_intel_gpu_drivers()?;
    }
    if Device::has_intel_cpu() {
      install_intel_webcam_drivers()?;
      install_intel_battery_optimizer()?;
    }

    if Device::has_amd_gpu() {
      install_rocm()?;
    }

    if Device::has_nvidia_gpu() {
      install_nvidia_gpu_drivers()?;
    }

    install_broadcom_fingerprint_drivers()?;

    Ok(())
  }
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

fn install_intel_gpu_drivers() -> Result<bool> {
  if Dnf::is_installed("intel-media-driver") {
    println!("Skipping Intel GPU drivers: already installed");
    return Ok(true);
  }
  if !ask("Install Intel GPU drivers?", true) {
    return Ok(false);
  }
  println!("Installing Intel GPU drivers...");
  add_to_render_video_groups()?;
  Dnf::install(&[
    "intel-media-driver",
    "libva-intel-driver",
    "mesa-libOpenCL",
    "intel-opencl",
  ])?;
  Ok(true)
}

fn install_intel_webcam_drivers() -> Result<bool> {
  if Dnf::is_installed("ipu6-camera-hal") {
    println!("Skipping Intel webcam drivers: already installed");
    return Ok(true);
  }
  if !ask("Install Intel webcam drivers?", true) {
    return Ok(false);
  }
  println!("Installing Intel webcam drivers...");
  Dnf::install(&[
    "intel-media-driver",
    "intel-vision",
    "akmod-intel-ipu6",
    "ipu6-camera-bins",
    "ipu6-camera-hal",
    "gstreamer1-plugins-icamerasrc",
    "akmod-v4l2loopback",
    "v4l2-relayd",
    "libcamera",
    "libcamera-gstreamer",
    "libcamera-v4l2",
  ])?;
  println!("Your webcam should work after a reboot :)");
  Ok(true)
}

fn install_intel_battery_optimizer() -> Result<bool> {
  if Dnf::is_installed("intel-lpmd") {
    println!("Skipping Intel's battery optimizer: already installed");
    return Ok(true);
  }
  if !ask("Install Intel's battery optimizer?", true) {
    return Ok(false);
  }
  println!("Installing Intel's battery optimizer...");
  Dnf::install(&["intel-lpmd"])?;
  run_interactively("sudo", &["systemctl", "enable", "--now", "intel_lpmd"])?;
  run_interactively("sudo", &["intel_lpmd_control", "AUTO"])?;
  Ok(true)
}

fn install_rocm() -> Result<bool> {
  if Dnf::is_installed("rocm") {
    println!("Skipping AMD ROCm: already installed");
    return Ok(true);
  }
  if !ask("Install AMD ROCm?", true) {
    return Ok(false);
  }
  println!("Installing AMD ROCm...");
  add_to_render_video_groups()?;
  Dnf::install(&["rocm"])?;
  Ok(true)
}

fn add_to_render_video_groups() -> Result<bool> {
  let user = var("LOGNAME").unwrap_or_else(|_| var("USER").unwrap());
  let groups = run_output("groups", &[&user])?
    .split_whitespace()
    .map(str::to_owned)
    .collect::<HashSet<String>>();
  if groups.contains("render") && groups.contains("video") {
    return Ok(true);
  }
  run_interactively("sudo", &["usermod", "-aG", "render,video", &user])?;
  Ok(true)
}

fn install_nvidia_gpu_drivers() -> Result<bool> {
  if Dnf::is_installed("libva-nvidia-driver") {
    println!("Skipping Nvidia drivers: already installed");
    return Ok(true);
  }
  if !ask("Install (proprietary) Nvidia drivers?", true) {
    return Ok(false);
  }
  println!("Installing Nvidia drivers...");
  Dnf::install(&[
    "akmod-nvidia",
    "xorg-x11-drv-nvidia-cuda",
    "libva-nvidia-driver.{i686,x86_64}",
  ])?;
  Ok(true)
}

fn install_broadcom_fingerprint_drivers() -> Result<bool> {
  let lsusb = run_output("lsusb", &[])?;
  let has_older_broadcom = lsusb.contains("0a5c:584");
  let has_newer_broadcom = lsusb.contains("0a5c:586");
  if !has_older_broadcom && !has_newer_broadcom {
    return Ok(false);
  }
  let driver = if has_older_broadcom {
    "libfprint-2-tod1-broadcom"
  } else {
    "libfprint-2-tod1-broadcom-cv3plus"
  };
  if Dnf::is_installed(driver) {
    println!("Skipping Broadcom fingerprint drivers: already installed");
    return Ok(true);
  }
  if !ask("Install Broadcom fingerprint drivers?", true) {
    return Ok(false);
  }
  println!("Installing Broadcom fingerprint drivers...");
  run_interactively(
    "sudo",
    &["dnf", "copr", "enable", "grahamwhiteuk/libfprint-tod"],
  )?;
  Dnf::swap(&["libfprint", "libfprint-tod"])?;
  Dnf::install(&[driver])?;
  Ok(true)
}
