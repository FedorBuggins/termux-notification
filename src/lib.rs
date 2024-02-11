//! Display a system notification.
//!
//! ```no_run
//! use std::io;
//!
//! use termux_notification::TermuxNotification;
//!
//! fn main() -> io::Result<()> {
//!   TermuxNotification::new()
//!     .title("Foo")
//!     .content("Bar")
//!     .show()?;
//!   Ok(())
//! }
//! ```
//!
//! ## Feature flags
//!
//! - `callbacks`

#[cfg(any(doc, feature = "callbacks"))]
pub mod callbacks;
mod options;
pub mod remove_handle;

use std::{
  collections::HashMap,
  io,
  process::{Command, Output},
};

use remove_handle::RemoveHandle;

/// Builder for `termux-notification` command
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TermuxNotification {
  args: HashMap<&'static str, Option<String>>,
}

impl TermuxNotification {
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Notification id (will overwrite any previous notification with the same id)
  pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
    self.args.insert(options::ID, Some(id.into()));
    self
  }

  /// Notification title to show
  pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
    self.args.insert(options::TITLE, Some(title.into()));
    self
  }

  /// Content to show in the notification.
  pub fn content(&mut self, content: impl Into<String>) -> &mut Self {
    self.args.insert(options::CONTENT, Some(content.into()));
    self
  }

  /// Set the icon that shows up in the status bar.
  /// View available icons at `https://material.io/resources/icons/`
  /// (default icon: `event_note`)
  pub fn icon(&mut self, icon: impl Into<String>) -> &mut Self {
    self.args.insert(options::ICON, Some(icon.into()));
    self
  }

  /// Do not alert when the notification is edited
  pub fn alert_once(&mut self, alert_once: bool) -> &mut Self {
    let k = options::ALERT_ONCE;
    if alert_once {
      self.args.insert(k, None);
    } else {
      self.args.remove(k);
    }
    self
  }

  /// Pin the notification
  pub fn ongoing(&mut self, ongoing: bool) -> &mut Self {
    let k = options::ONGOING;
    if ongoing {
      self.args.insert(k, None);
    } else {
      self.args.remove(k);
    }
    self
  }

  /// Action to execute when pressing the notification
  pub fn action(&mut self, action: impl Into<String>) -> &mut Self {
    self.args.insert(options::ACTION, Some(action.into()));
    self
  }

  /// Action to execute when the the notification is cleared
  pub fn on_delete(
    &mut self,
    on_delete: impl Into<String>,
  ) -> &mut Self {
    self.args.insert(options::ON_DELETE, Some(on_delete.into()));
    self
  }

  /// Text and action for first notification button
  pub fn button1(
    &mut self,
    label: impl Into<String>,
    action: impl Into<String>,
  ) -> &mut Self {
    self.args.insert(options::BUTTON1, Some(label.into()));
    self
      .args
      .insert(options::BUTTON1_ACTION, Some(action.into()));
    self
  }

  /// Text and action for second notification button
  pub fn button2(
    &mut self,
    label: impl Into<String>,
    action: impl Into<String>,
  ) -> &mut Self {
    self.args.insert(options::BUTTON2, Some(label.into()));
    self
      .args
      .insert(options::BUTTON2_ACTION, Some(action.into()));
    self
  }

  /// Text and action for third notification button
  pub fn button3(
    &mut self,
    label: impl Into<String>,
    action: impl Into<String>,
  ) -> &mut Self {
    self.args.insert(options::BUTTON3, Some(label.into()));
    self
      .args
      .insert(options::BUTTON3_ACTION, Some(action.into()));
    self
  }

  /// Shows notification via `termux-notification` command
  ///
  /// # Errors
  ///
  /// Returns an error if command status not success
  pub fn show(&self) -> io::Result<RemoveHandle> {
    ensure_success(&self.to_command().output()?)?;
    let id = self.args.get(options::ID);
    Ok(RemoveHandle::new(id.cloned().flatten()))
  }

  /// Builds `termux-notification` command
  #[must_use]
  pub fn to_command(&self) -> Command {
    let mut cmd = Command::new("termux-notification");
    for (key, val) in &self.args {
      cmd.arg(key);
      if let Some(val) = val {
        cmd.arg(val);
      }
    }
    cmd
  }
}

/// # Errors
///
/// Returns an error if command status not success
fn ensure_success(output: &Output) -> io::Result<()> {
  if output.status.success() {
    Ok(())
  } else {
    Err(io::Error::other(stdout_stderr(output)))
  }
}

fn stdout_stderr(output: &Output) -> String {
  let stdout = String::from_utf8_lossy(&output.stdout);
  let stderr = String::from_utf8_lossy(&output.stderr);
  let out = &mut vec![stdout, stderr];
  out.retain(|s| !s.is_empty());
  out.join("\n")
}
