use std::{collections::HashMap, hint::black_box, time::Duration};

use crate::{app::Message, prelude::*};

use iced::{Task, futures::SinkExt, stream};
use kanal::Sender;
use rdev::{Event, EventType, listen};
use tokio::time::{self, sleep};

/// Spawn a task that listens for keystrokes at an interval
///
/// # Parameters
/// - `interval`: The time between each message containing the latest keystrokes
#[instrument(skip_all, level = Level::DEBUG)]
pub fn task_run(interval: Duration) -> Task<Message> {
  Task::stream(stream::channel(8, async move |mut output| {
    let (event_sender, event_receiver) = kanal::unbounded::<Event>();

    tokio::task::spawn_blocking(move || listener_thread(event_sender));

    let mut received_events = Vec::with_capacity(64);

    loop {
      time::sleep(interval).await;

      let mut key_occurrences: HashMap<rdev::Key, u32> = HashMap::new();

      if !event_receiver.is_empty() {
        event_receiver.drain_into(&mut received_events).unwrap();

        for event in &received_events {
          if let EventType::KeyPress(key) = event.event_type {
            *key_occurrences.entry(key).or_default() += 1;
          }
        }

        received_events.clear();

        let active_window = x_win::get_active_window().unwrap();

        output
          .send(Message::KeyboardEvents {
            key_occurrences,
            active_window: active_window,
          })
          .await
          .unwrap();
      }
    }
  }))
}

#[instrument(skip_all, level = Level::TRACE)]
fn listener_thread(event_sender: Sender<Event>) {
  let listening_result = listen(move |event: Event| {
    if !matches!(event.event_type, EventType::MouseMove { .. }) {
      event_sender.send(event).unwrap();
    }
  });

  if let Err(error) = listening_result {
    error!("error listening to system events: {:?}", error);
    panic!("error listening to system events: {:?}", error);
  }
}
