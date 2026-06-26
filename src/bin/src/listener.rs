use crate::prelude::*;

use kanal::Sender;
use rdev::{Event, EventType, listen};

pub fn listener_thread(event_sender: Sender<Event>, enable_mouse_motion: bool) {
  let closure = if enable_mouse_motion {
    move |event: Event| {
      event_sender.send(event).unwrap();
    }
  } else {
    move |event: Event| {
      if !matches!(event.event_type, EventType::MouseMove { .. }) {
        event_sender.send(event).unwrap();
      }
    }
  };

  // This will block the thread.
  if let Err(error) = listen(closure) {
    error!("error listening to system events: {:?}", error);
    panic!("error listening to system events: {:?}", error);
  }
}
