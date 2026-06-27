use std::time::Duration;

use crate::{app::Message, prelude::*};

use iced::{Task, futures::SinkExt, stream};
use kanal::Sender;
use rdev::{Event, EventType, listen};

pub fn task() -> Task<Message> {
  Task::stream(stream::channel(8, async move |mut output| {
    let (event_sender, event_receiver) = kanal::unbounded::<Event>();

    std::thread::spawn(move || listener_thread(event_sender, false));

    loop {
      let mut received_events = Vec::with_capacity(32);

      tokio::time::sleep(Duration::from_millis(2000)).await;

      if !event_receiver.is_empty() {
        event_receiver.drain_into(&mut received_events).unwrap();

        output.send(Message::KeyboardEvents(received_events)).await.unwrap();
      }
    }
  }))
}

fn listener_thread(event_sender: Sender<Event>, enable_mouse_motion: bool) {
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
