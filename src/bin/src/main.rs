#[allow(unused_imports)]
mod prelude;

use std::time::Duration;

use rdev::{Event, EventType};

use crate::{listener::listener_thread, prelude::*};

mod listener;

fn main() {
  let (event_sender, event_receiver) = kanal::unbounded::<Event>();

  std::thread::spawn(move || listener_thread(event_sender, false));

  let mut received_events = Vec::new();
  loop {
    std::thread::sleep(Duration::from_millis(2000));

    if !event_receiver.is_empty() {
      event_receiver.drain_into(&mut received_events).unwrap();

      println!("{:?}", received_events);

      received_events.clear();
    }
  }
}
