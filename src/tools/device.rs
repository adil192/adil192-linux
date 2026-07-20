use std::fs;

use cached::once;
use cmd_lib::run_fun;

#[once()]
fn get_gpu_info() -> String {
  run_fun!(lspci "-mm" | grep -E -i "vga|display|3d")
    .unwrap()
    .to_lowercase()
}
#[once()]
fn get_cpu_info() -> String {
  fs::read_to_string("/proc/cpuinfo").unwrap().to_lowercase()
}

pub struct Device;
impl Device {
  #[allow(unused)]
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
