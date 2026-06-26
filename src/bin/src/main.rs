#[allow(unused_imports)]
mod prelude;

use rdev::{Event, EventType};

use crate::{listener::listener_thread, prelude::*};

mod listener;

fn main() {
  let (event_sender, event_receiver) = kanal::unbounded::<Event>();

  std::thread::spawn(move || listener_thread(event_sender, false));

  loop {
    let event = event_receiver.recv().unwrap();

    if let Event {
      event_type: EventType::KeyPress(key_press),
      ..
    } = event
    {
      println!("{:?}", key_press);
    }
  }
}
