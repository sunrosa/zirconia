use std::{any::Any, collections::HashMap, hint::black_box, io::ErrorKind, ops::ControlFlow, time::Duration};

use crate::{app::Message, prelude::*};

use iced::{Task, futures::SinkExt, stream};
use kanal::Sender;
use rdev::{Event, EventType, listen};
use tokio::{
  task::JoinHandle,
  time::{self, sleep},
};

/// Spawn a task that listens for keystrokes at an interval
///
/// # Parameters
/// - `interval`: The time between each message containing the latest keystrokes
#[instrument(skip_all, level = Level::DEBUG)]
pub fn task_run(interval: Duration) -> (Task<Message>, JoinHandle<()>) {
  // This inner channel (the kanal channel) receives messages for EVERY key event.
  let (event_sender, event_receiver) = kanal::unbounded::<Event>();

  // The listener sends messages from a blocking thread, as rdev doesn't have an asynchronous API.
  debug!("spawning a blocking thread via tokio's threadpool to listen to key events");
  let listener_handle = tokio::task::spawn_blocking(move || listener_thread(event_sender));

  // The outer channel (stream channel) only sends messages at [`interval`], to keep the UI thread from updating every global keystroke.
  let task = Task::stream(stream::channel(8, async move |mut output| {
    let mut received_events = Vec::with_capacity(64);

    loop {
      // If you aren't receiving lints in this entire function, update rust-analyzer and it will work again. It's because the line below causes this block to become an actual async closure.
      time::sleep(interval).await;

      let mut key_occurrences: HashMap<rdev::Key, u32> = HashMap::new();

      // No events are sent up to the UI unless new events are present.
      if !event_receiver.is_empty() {
        event_receiver.drain_into(&mut received_events).unwrap();

        for event in &received_events {
          if let EventType::KeyPress(key) = event.event_type {
            *key_occurrences.entry(key).or_default() += 1;
          }
        }

        received_events.clear();

        let active_window = match active_window() {
          Some(value) => value,
          None => continue,
        };

        output
          .send(Message::KeyboardEvents {
            key_occurrences,
            active_window: active_window,
          })
          .await
          .unwrap();
      }
    }
  }));

  (task, listener_handle)
}

/// If [`None`] is returned, the error was insignificant/recoverable (like permission denied to access window).
///
/// # Panics
/// If the error is unrecoverable, it **panics**.
#[inline]
#[instrument(skip_all, level = Level::TRACE)]
fn active_window() -> Option<x_win::WindowInfo> {
  Some(match x_win::get_active_window() {
    Ok(o) => o,
    Err(e) => {
      if let Some(io_error) = e.downcast_ref::<no_std_io2::io::Error>() {
        match io_error.kind() {
          ErrorKind::PermissionDenied => {
            warn!("permission denied to access window: {io_error:?}");
            return None;
          }
          _ => panic!("{io_error:?}"),
        }
      } else {
        panic!("{e:?}");
      }
    }
  })
}

#[instrument(skip_all, level = Level::TRACE)]
fn listener_thread(event_sender: Sender<Event>) {
  let listening_result = listen(move |event: Event| {
    // We are ignoring mouse movements with this condition
    if !matches!(event.event_type, EventType::MouseMove { .. }) {
      match event_sender.send(event) {
        Ok(()) => {}
        Err(e) => {
          // If the channel has closed, there's nothing we can do from here to kill this thread. Just hope it gets closed by the OS since main has probably closed if the channel is closed.
          warn!("high-frequency key event receiver has hung up (main thread probably closed): {e:?}")
        }
      }
    }
  });

  if let Err(error) = listening_result {
    error!("error listening to system events: {:?}", error);
    panic!("error listening to system events: {:?}", error);
  }
}
