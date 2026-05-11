use std::process::Command;

/// Runs the command with inherited stdin/stdout/stderr.
///
/// This lets the user interact with the command as if they were running it directly in the shell,
/// but we can't receive the output of the command programmatically.
///
/// If you need to capture the output of the command, use [`run_output`] instead.
pub fn run_interactively(command: &str, args: &[&str]) {
  let Ok(status) = Command::new(command).args(args).status() else {
    panic!("Command failed to start: {command} {args:?}");
  };
  if !status.success() {
    panic!("Command failed with status {status:?}: {command} {args:?}");
  }
}

/// Runs the command and returns the output (stdout) as a string.
///
/// The process does not inherit stdin, so the user cannot interact with the command.
///
/// If you need to run a command interactively, use [`run_interactively`] instead.
pub fn run_output(command: &str, args: &[&str]) -> String {
  let Ok(output) = Command::new(command).args(args).output() else {
    panic!("Command failed to start: {command} {args:?}");
  };
  if !output.status.success() {
    panic!(
      "Command failed with status {status:?}: {command} {args:?}\n{output:?}",
      status = output.status
    );
  }
  String::from_utf8_lossy(&output.stdout).to_string()
}
