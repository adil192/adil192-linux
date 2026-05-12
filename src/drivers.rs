use anyhow::Result;

use crate::tools::{dnf::Dnf, dnf_repos::DnfRepos};

pub fn install() -> Result<()> {
  if !Dnf::exists() {
    println!("DNF is not available, skipping drivers installation.");
    return Ok(());
  }

  DnfRepos::add_rpmfusion_repos()?;
  DnfRepos::add_terra_repos()?;
  DnfRepos::add_ultramarine_repos()?;

  Ok(())
}
