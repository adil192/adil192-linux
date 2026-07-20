use std::io::{Write, stdin, stdout};
use std::process::Command;

pub mod device;
pub mod dnf;
pub mod dnf_repos;
pub mod flatpak;

pub fn is_exe_in_path(exe: &str) -> bool {
  let Ok(location) = Command::new("which").arg(exe).output() else {
    return false;
  };
  !location.stdout.is_empty()
}

/// Asks the user a yes/no question.
/// - Returns true if the user responds with Y, false with N.
/// - Returns `default_response` if the user presses enter without typing anything.
/// - Panics if the stdout/stdin operations fail.
pub fn ask(question: &str, default_response: bool) -> bool {
  try_ask(question, default_response).unwrap()
}

fn try_ask(question: &str, default_response: bool) -> anyhow::Result<bool> {
  let hint = if default_response { "Y/n" } else { "y/N" };
  let mut lock = stdout().lock();
  write!(lock, "{question} ({hint}): ")?;
  lock.flush()?;

  let mut response = String::new();
  loop {
    response.clear();
    stdin().read_line(&mut response)?;
    response = response.trim().to_lowercase();

    if response.is_empty() {
      return Ok(default_response);
    } else if response == "y" {
      return Ok(true);
    } else if response == "n" {
      return Ok(false);
    }
  }
}
