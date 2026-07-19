use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::tools::dnf::Dnf;
use crate::tools::{ask, run_output};

static REPOS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
fn get_repos() -> &'static Mutex<HashSet<String>> {
  REPOS.get_or_init(|| {
    let output = run_output("dnf", &["repolist", "--json"]).unwrap();
    let set = serde_json::from_str::<serde_json::Value>(&output)
      .unwrap()
      .as_array()
      .unwrap()
      .iter()
      .map(|repo| repo["id"].as_str().unwrap().to_owned())
      .collect();
    Mutex::new(set)
  })
}

pub struct DnfRepos;
impl DnfRepos {
  pub fn add_rpmfusion_repos() -> anyhow::Result<bool> {
    let mut repos = get_repos().lock().unwrap();
    if repos.contains("rpmfusion-free") && repos.contains("rpmfusion-nonfree") {
      println!("Skipping RPM Fusion repos: already added");
      return Ok(true);
    }
    if !ask("Add RPM Fusion repos?", true) {
      return Ok(false);
    }
    println!("Adding RPM Fusion repos...");
    let release = run_output("rpm", &["-E", "%fedora"])?;
    Dnf::install(&[
      &format!(
        "https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-{release}.noarch.rpm"
      ),
      &format!(
        "https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-{release}.noarch.rpm"
      ),
    ])?;
    repos.insert("rpmfusion-free".to_owned());
    repos.insert("rpmfusion-nonfree".to_owned());
    println!("Updating Appstream metadata");
    Dnf::update(&["@core"])?;
    Ok(true)
  }

  pub fn add_terra_repos() -> anyhow::Result<bool> {
    let mut repos = get_repos().lock().unwrap();
    if repos.contains("terra") {
      println!("Skipping Terra repos: already added");
      return Ok(true);
    }
    if !ask("Add Terra repos?", true) {
      return Ok(false);
    }
    println!("Adding Terra repos...");
    let release = run_output("rpm", &["-E", "%fedora"])?;
    Dnf::install(&[
      "--repofrompath",
      "terra,https://repos.fyralabs.com/terra$releasever",
      &format!("--setopt=\"terra.gpgkey=https://repos.fyralabs.com/terra{release}/key.asc\""),
      "terra-release",
    ])?;
    repos.insert("terra".to_owned());
    Ok(true)
  }
}
