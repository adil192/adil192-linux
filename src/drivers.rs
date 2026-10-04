use std::collections::HashSet;
use std::env::var;
use std::path::Path;

use anyhow::Result;
use cmd_lib::{run_cmd, run_fun};

use crate::tools::device::Device;
use crate::tools::dnf::Dnf;
use crate::tools::dnf_repos::DnfRepos;
use crate::tools::{ask, ask_to_install};

pub struct FedoraDrivers;
impl FedoraDrivers {
  /// Loosely based on https://rpmfusion.org/Howto/Multimedia
  pub fn install() -> Result<()> {
    if !Dnf::exists() {
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
  ask_to_install(
    "full-fat ffmpeg",
    || !Dnf::is_installed("ffmpeg-free") && Dnf::is_installed("libavcodec-freeworld"),
    || {
      Dnf::swap(&["ffmpeg-free", "ffmpeg", "--allowerasing"])?;
      Dnf::install(&["libavcodec-freeworld"])
    },
  )
}

fn install_gstreamer_plugins() -> Result<bool> {
  ask_to_install(
    "gstreamer plugins",
    || Dnf::is_installed("gstreamer1-plugins-ugly"),
    || {
      Dnf::update(&[
        "@multimedia",
        "--setopt=install_weak_deps=False",
        "--exclude=PackageKit-gstreamer-plugin",
      ])
    },
  )
}

fn add_mesa_copr() -> Result<bool> {
  let repo_file =
    Path::new("/etc/yum.repos.d/_copr:copr.fedorainfracloud.org:adil192:mesa-x86-64-v3.repo");
  if repo_file.exists() {
    println!("Skipping 'adil192/mesa-x86-64-v3' copr: already added");
    return Ok(true);
  }
  if Path::new("/etc/yum.repos.d/terra-mesa.repo").exists() {
    println!("Skipping 'adil192/mesa-x86-64-v3' copr: terra-mesa already added");
    return Ok(false);
  }
  if !ask(
    "Add my repo for faster Mesa (graphics driver) updates?",
    false,
  ) {
    return Ok(false);
  }
  println!("Adding my repo for faster Mesa (graphics driver) updates...");
  run_cmd!("sudo dnf copr enable adil192/mesa-x86-64-v3")?;
  println!("My builds will be installed the next time you run `sudo dnf update`.");
  Ok(true)
}

fn install_intel_gpu_drivers() -> Result<bool> {
  ask_to_install(
    "Intel GPU drivers",
    || Dnf::is_installed("intel-media-driver"),
    || {
      add_to_render_video_groups()?;
      Dnf::install(&[
        "intel-media-driver",
        "libva-intel-driver",
        "mesa-libOpenCL",
        "intel-opencl",
      ])
    },
  )
}

fn install_intel_webcam_drivers() -> Result<bool> {
  ask_to_install(
    "Intel webcam drivers",
    || Dnf::is_installed("ipu6-camera-hal"),
    || {
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
      Ok(())
    },
  )
}

fn install_intel_battery_optimizer() -> Result<bool> {
  ask_to_install(
    "Intel's battery optimizer",
    || Dnf::is_installed("intel-lpmd"),
    || {
      Dnf::install(&["intel-lpmd"])?;
      run_cmd!(sudo systemctl enable --now intel_lpmd)?;
      run_cmd!(sudo intel_lpmd_control AUTO)?;
      Ok(())
    },
  )
}

fn install_rocm() -> Result<bool> {
  ask_to_install(
    "AMD ROCm",
    || Dnf::is_installed("rocm"),
    || {
      add_to_render_video_groups()?;
      Dnf::install(&["rocm"])
    },
  )
}

fn add_to_render_video_groups() -> Result<bool> {
  let user = var("LOGNAME").unwrap_or_else(|_| var("USER").unwrap());
  let groups = run_fun!(groups $user)?
    .split_whitespace()
    .map(str::to_owned)
    .collect::<HashSet<String>>();
  if groups.contains("render") && groups.contains("video") {
    return Ok(true);
  }
  run_cmd!(sudo usermod -aG render,video $user)?;
  Ok(true)
}

fn install_nvidia_gpu_drivers() -> Result<bool> {
  ask_to_install(
    "Nvidia drivers",
    || Dnf::is_installed("libva-nvidia-driver"),
    || {
      Dnf::install(&[
        "akmod-nvidia",
        "xorg-x11-drv-nvidia-cuda",
        "libva-nvidia-driver.{i686,x86_64}",
      ])
    },
  )
}

fn install_broadcom_fingerprint_drivers() -> Result<bool> {
  let lsusb = run_fun!(lsusb)?;
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

  ask_to_install(
    "Broadcom fingerprint drivers",
    || Dnf::is_installed(driver),
    || {
      run_cmd!(sudo dnf copr enable grahamwhiteuk/libfprint-tod)?;
      Dnf::swap(&["libfprint", "libfprint-tod"])?;
      Dnf::install(&[driver])
    },
  )
}
