#[allow(unused_imports)]
mod prelude;

use rdev::Event;

use crate::{listener::listener_thread, prelude::*};

mod listener;

fn main() {
  let (event_sender, event_receiver) = kanal::unbounded::<Event>();

  std::thread::spawn(move || listener_thread(event_sender));

  loop {
    let event = event_receiver.recv();
    println!("{:?}", event);
  }
}
