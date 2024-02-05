use std::{io, sync::mpsc};

use termux_notification::TermuxNotification;

#[derive(Clone, Copy)]
enum Action {
  Increment,
  Decrement,
  Reset,
  Exit,
}

fn main() -> io::Result<()> {
  termux_notification::callbacks::init_socket();

  let (tx, rx) = mpsc::channel();

  let mut state = 0;
  let mut old = None;

  loop {
    let cb = |action| {
      let tx = tx.clone();
      move || tx.send(action).unwrap()
    };

    let new = TermuxNotification::new()
      .id("example")
      .title("Callbacks Example")
      .content(format!("Counter: {state} (tap to reset)"))
      .button1_fn("INCREMENT", cb(Action::Increment))
      .button2_fn("DECREMENT", cb(Action::Decrement))
      .action_fn(cb(Action::Reset))
      .on_delete_fn(cb(Action::Exit))
      .to_command()
      .spawn()?; // spawn for smooth update

    if let Some(mut old) = old.replace(new) {
      old.kill()?;
    }

    match rx.recv().map_err(io::Error::other)? {
      Action::Increment => state += 1,
      Action::Decrement => state -= 1,
      Action::Reset => state = 0,
      Action::Exit => return Ok(()),
    };
  }
}
