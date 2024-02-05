use std::{io, process, thread, time::Duration};

use termux_notification::TermuxNotification;

fn main() -> io::Result<()> {
  termux_notification::callbacks::init_socket();

  let remove_handle = TermuxNotification::new()
    .id("example")
    .title("Termux Notification Example")
    .button1_fn("ECHO", || println!("Hello"))
    .on_delete_fn(|| {
      println!("Notification deleted");
      process::exit(0);
    })
    .show()?;

  thread::sleep(Duration::from_secs(60));

  remove_handle.remove()
}
