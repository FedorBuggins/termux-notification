//! Provides callbacks to notifications by socket connection.
//!
//! # Requirements
//!
//! - `netcat-openbsd` package installed
//! - `callbacks` feature enabled
//! - `id` provided for notification with callbacks
//!
//! # Usage
//!
//! On application start initialize callbacks socket.
//!
//! ```no_run
//! termux_notification::callbacks::init_socket();
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use std::{io, process, thread, time::Duration};
//!
//! use termux_notification::TermuxNotification;
//!
//! fn main() -> io::Result<()> {
//!   termux_notification::callbacks::init_socket();
//!
//!   let remove_handle = TermuxNotification::new()
//!     .id("example")
//!     .title("Termux Notification Example")
//!     .button1_fn("ECHO", || println!("Hello"))
//!     .on_delete_fn(|| {
//!       println!("Notification deleted");
//!       process::exit(0);
//!     })
//!     .show()?;
//!
//!   thread::sleep(Duration::from_secs(60));
//!
//!   remove_handle.remove()
//! }
//! ```

mod callback_key;

use std::{
  collections::HashMap,
  env,
  io::{self, Read},
  os::unix::net::UnixListener,
  path::PathBuf,
  process,
  str::FromStr,
  sync::{Mutex, Once},
  thread,
};

use crate::{options, TermuxNotification};

use self::callback_key::CallbackKey;

type CallbackMap = HashMap<CallbackKey, Box<dyn Fn() + Send>>;

static CALLBACK_MAP: Mutex<Option<CallbackMap>> = Mutex::new(None);

/// Creates socket and listen it at new thread to handle notification callbacks
///
/// # Panics
///
/// Panics if can't create socket.
/// Spawned thread panics on receive message error.
pub fn init_socket() {
  static INIT: Once = Once::new();
  INIT.call_once(|| {
    let socket = UnixListener::bind(socket_path()).unwrap();
    thread::spawn(move || loop {
      let msg = recv_message(&socket).unwrap();
      let key = CallbackKey::from_str(&msg);
      let Ok(key) = key else { continue };
      let cb_map = &mut CALLBACK_MAP.lock().unwrap();
      let cb_map = cb_map.get_or_insert_with(HashMap::new);
      let Some(f) = cb_map.get(&key) else { continue };
      f();
      if key.is_finish_trigger() {
        cb_map.retain(|k, _| k.id() != key.id());
      }
    });
  });
}

fn socket_path() -> PathBuf {
  let pid = process::id();
  env::temp_dir().join(format!("termux_notification.{pid}.socket"))
}

fn recv_message(socket: &UnixListener) -> io::Result<String> {
  let (mut connection, _) = socket.accept()?;
  let buf = &mut String::new();
  connection.read_to_string(buf)?;
  Ok(buf.trim().to_owned())
}

impl TermuxNotification {
  /// Action to execute when pressing the notification
  ///
  /// # Panics
  ///
  /// Panics if notification id not provided
  pub fn action_fn<F>(&mut self, f: F) -> &mut Self
  where
    F: Fn() + Send + 'static,
  {
    let cmd = self.on(options::ACTION, f);
    self.action(cmd)
  }

  /// Action to execute when the the notification is cleared
  ///
  /// # Panics
  ///
  /// Panics if notification id not provided
  pub fn on_delete_fn<F>(&mut self, f: F) -> &mut Self
  where
    F: Fn() + Send + 'static,
  {
    let cmd = self.on(options::ON_DELETE, f);
    self.on_delete(cmd)
  }

  /// Text and action for first notification button
  ///
  /// # Panics
  ///
  /// Panics if notification id not provided
  pub fn button1_fn<L, F>(&mut self, label: L, f: F) -> &mut Self
  where
    L: Into<String>,
    F: Fn() + Send + 'static,
  {
    let cmd = self.on(options::BUTTON1, f);
    self.button1(label, cmd)
  }

  /// Text and action for second notification button
  ///
  /// # Panics
  ///
  /// Panics if notification id not provided
  pub fn button2_fn<L, F>(&mut self, label: L, f: F) -> &mut Self
  where
    L: Into<String>,
    F: Fn() + Send + 'static,
  {
    let cmd = self.on(options::BUTTON2, f);
    self.button2(label, cmd)
  }

  /// Text and action for third notification button
  ///
  /// # Panics
  ///
  /// Panics if notification id not provided
  pub fn button3_fn<L, F>(&mut self, label: L, f: F) -> &mut Self
  where
    L: Into<String>,
    F: Fn() + Send + 'static,
  {
    let cmd = self.on(options::BUTTON3, f);
    self.button3(label, cmd)
  }

  fn on<F>(&mut self, trigger: &str, f: F) -> String
  where
    F: Fn() + Send + 'static,
  {
    let id = self.get_id_unchecked();
    let key = CallbackKey::new(id, trigger.to_owned());
    let socket = socket_path().to_string_lossy().to_string();
    let cmd = format!(r#"echo "{key}" | nc -UN {socket}"#);
    CALLBACK_MAP
      .lock()
      .unwrap()
      .get_or_insert_with(HashMap::new)
      .insert(key, Box::new(f));
    cmd
  }

  fn get_id_unchecked(&self) -> String {
    self
      .args
      .get(options::ID)
      .cloned()
      .flatten()
      .expect("id not provided")
  }
}
