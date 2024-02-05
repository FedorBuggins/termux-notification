use std::{io, process::Command};

use super::ensure_success;

/// Remove a notification previously shown with id
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveHandle {
  id: Option<String>,
}

impl RemoveHandle {
  #[must_use]
  pub fn new(id: Option<String>) -> Self {
    Self { id }
  }

  #[must_use]
  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  /// Runs `termux-notification-remove` command if `id` present
  ///
  /// # Errors
  ///
  /// Returns an error if command status not success
  pub fn remove(&self) -> io::Result<()> {
    if let Some(mut cmd) = self.to_command() {
      ensure_success(&cmd.output()?)?;
    }
    Ok(())
  }

  /// Builds `termux-notification-remove` command
  #[must_use]
  pub fn to_command(&self) -> Option<Command> {
    let mut cmd = Command::new("termux-notification-remove");
    cmd.arg(self.id()?);
    Some(cmd)
  }
}
