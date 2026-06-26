use crate::prelude::*;

use kanal::Sender;
use rdev::{Event, EventType, listen};

pub fn listener_thread(event_sender: Sender<Event>) {
  // This will block the thread.
  if let Err(error) = listen(move |event| {
    if let EventType::KeyPress(..) = event.event_type {
      event_sender.send(event).unwrap();
    }
  }) {
    error!("error listening to system events: {:?}", error);
    panic!("error listening to system events: {:?}", error);
  }
}
