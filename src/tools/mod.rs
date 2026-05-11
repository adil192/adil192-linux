use std::{
  io::{Write, stdin, stdout},
  process::Command,
};

pub mod dnf;
pub mod flatpak;

pub fn is_exe_in_path(exe: &str) -> bool {
  let Ok(location) = Command::new("which").arg(exe).output() else {
    return false;
  };
  !location.stdout.is_empty()
}

/// Runs the command with inherited stdin/stdout/stderr.
///
/// This lets the user interact with the command as if they were running it directly in the shell,
/// but we can't receive the output of the command programmatically.
///
/// If you need to capture the output of the command, use [`run_output`] instead.
pub fn run_interactively(command: &str, args: &[&str]) -> anyhow::Result<()> {
  let Ok(status) = Command::new(command).args(args).status() else {
    panic!("Command failed to start: {command} {args:?}");
  };
  if !status.success() {
    anyhow::bail!("Command failed with status {status:?}: {command} {args:?}")
  }
  Ok(())
}

/// Runs the command and returns the output (stdout) as a string.
///
/// The process does not inherit stdin, so the user cannot interact with the command.
///
/// If you need to run a command interactively, use [`run_interactively`] instead.
pub fn run_output(command: &str, args: &[&str]) -> anyhow::Result<String> {
  let Ok(output) = Command::new(command).args(args).output() else {
    panic!("Command failed to start: {command} {args:?}");
  };
  if !output.status.success() {
    anyhow::bail!(
      "Command failed with status {status:?}: {command} {args:?}\n{output:?}",
      status = output.status
    )
  }
  Ok(String::from_utf8_lossy(&output.stdout).to_string())
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
