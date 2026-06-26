use crate::prelude::*;

use kanal::Sender;
use rdev::{Event, EventType, listen};

pub fn listener_thread(event_sender: Sender<Event>, enable_mouse_motion: bool) {
  let listening_result = if enable_mouse_motion {
    listen(move |event: Event| {
      event_sender.send(event).unwrap();
    })
  } else {
    listen(move |event: Event| {
      if !matches!(event.event_type, EventType::MouseMove { .. }) {
        event_sender.send(event).unwrap();
      }
    })
  };

  if let Err(error) = listening_result {
    error!("error listening to system events: {:?}", error);
    panic!("error listening to system events: {:?}", error);
  }
}
