use std::sync::LazyLock;

use crate::tools::is_exe_in_path;

pub static EXISTS: LazyLock<bool> = LazyLock::new(|| is_exe_in_path("flatpak"));

pub fn installed(id: &str) -> bool {
  todo!();
}

pub fn install(ids: &[&str]) -> anyhow::Result<()> {
  todo!();
}
