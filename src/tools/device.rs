use std::fs;

use cached::proc_macro::once;

use crate::tools::run_output;

#[once(sync_writes = true)]
fn get_gpu_info() -> String {
  run_output("lspci", &["-mm"])
    .unwrap()
    .lines()
    .filter(|line| {
      let lowercase = line.to_lowercase();
      lowercase.contains("vga") || lowercase.contains("display") || lowercase.contains("3d")
    })
    .collect()
}
#[once(sync_writes = true)]
fn get_cpu_info() -> String {
  fs::read_to_string("/proc/cpuinfo").unwrap().to_lowercase()
}

pub struct Device;
impl Device {
  pub fn has_amd_cpu() -> bool {
    get_cpu_info().contains("amd")
  }
  pub fn has_intel_cpu() -> bool {
    get_cpu_info().contains("intel")
  }

  pub fn has_amd_gpu() -> bool {
    get_gpu_info().contains("amd")
  }
  pub fn has_intel_gpu() -> bool {
    get_gpu_info().contains("intel")
  }
  pub fn has_nvidia_gpu() -> bool {
    get_gpu_info().contains("nvidia")
  }
}
