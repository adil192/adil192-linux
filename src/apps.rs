use crate::tools::{ask::ask, run::run_interactively};
use std::path::Path;

pub fn install() {
  install_cosmic_copr();
}

fn install_cosmic_copr() {
  let repo_file =
    Path::new("/etc/yum.repos.d/_copr:copr.fedorainfracloud.org:adil192:cosmic-epoch.repo");
  if repo_file.exists() {
    return;
  }
  if !ask("Install my repo for faster COSMIC updates?", true) {
    return;
  }
  println!("Installing my repo for faster COSMIC updates...");
  run_interactively("sudo", &["dnf", "copr", "enable", "adil192/cosmic-epoch"]);
  println!("My builds will be installed the next time you run `sudo dnf update`.")
}
