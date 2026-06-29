use std::{any::Any, collections::HashMap, hint::black_box, io::ErrorKind, ops::ControlFlow, time::Duration};

use crate::{
  app::{Message, PressCount},
  prelude::*,
};

use iced::{Task, futures::SinkExt, stream};
use kanal::{Receiver, Sender};
use rdev::{Event, EventType, listen};
use tokio::{
  task::JoinHandle,
  time::{self, sleep},
};

// FIXME if the listener thread panics, main does not unwind. We need to catch the unwind, and notify the UI loop that it's lost its listener when it panics.

/// Spawn a task that listens for keystrokes at an interval
///
/// # Parameters
/// - `interval`: The time between each message containing the latest keystrokes
#[instrument(skip_all, level = Level::DEBUG)]
pub fn task_run(interval: Duration) -> Task<Message> {
  // This inner channel (the kanal channel) receives messages for EVERY key event.
  let (event_sender, event_receiver) = kanal::unbounded::<Event>();

  // The outer channel (stream channel) only sends messages at [`interval`], to keep the UI thread from updating every global keystroke.
  let task = Task::stream(stream::channel(8, async move |mut output| {
    let mut received_events = Vec::with_capacity(64);

    // The listener sends messages from a blocking thread. Do NOT use a tokio thread for this (even a tokio blocking thread), it will lock up the program forever.
    debug!("spawning a blocking os thread to listen to key events");
    let listener_thread_handle = std::thread::spawn(move || listener_thread(event_sender));
    output
      .send(Message::NewCriticalThread(listener_thread_handle))
      .await
      .unwrap();

    loop {
      // If you aren't receiving lints in this entire function, update rust-analyzer and it will work again. It's because the line below causes this block to become an actual async closure.
      // NOTE Instead of awaiting on a timer like this, we can instead await receiving an event, and THEN sleep after it's processed. That means the first keypress gets immediately processed, while the following keys are processed next iteration, though maybe that's worse. Currently it's completely out-of-sync to keypresses.
      time::sleep(interval).await;

      let mut key_occurrences: HashMap<rdev::Key, PressCount> = HashMap::new();

      // No events are sent up to the UI unless new events are present.
      if !event_receiver.is_empty() {
        event_receiver.drain_into(&mut received_events).unwrap();

        for event in &received_events {
          if let EventType::KeyPress(key) = event.event_type {
            *key_occurrences.entry(key).or_default() += PressCount(1);
          }
        }

        received_events.clear();

        let active_window = match active_window() {
          Some(value) => value,
          None => continue,
        };

        let active_layout = get_layout();

        panic!("TEST");

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

  task
}

/// If [`None`] is returned, the error was insignificant/recoverable (like permission denied to access window).
///
/// # Panics
/// If the error is unrecoverable, it **panics**.
#[inline]
#[instrument(skip_all, level = Level::TRACE)]
fn active_window() -> Option<x_win::WindowInfo> {
  match x_win::get_active_window() {
    Ok(o) => Some(o),
    Err(e) => {
      if let Some(io_error) = e.downcast_ref::<no_std_io2::io::Error>() {
        match io_error.kind() {
          ErrorKind::PermissionDenied => {
            warn!("permission denied to access window: {io_error:?}");
            None
          }
          _ => panic!("{io_error:?}"),
        }
      } else {
        // WARN Sometimes strange untyped errors occur, like `Not possible to recver pid for the window when calling _NET_WM_PID!`. Can't do much to match on them. Annoying.
        error!("{e:?}");
        None
      }
    }
  }
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
    panic!("error listening to system events: {:?}", error);
  }
}

/// Get the current keyboard layout. Returns values like `us` and `ru`.
fn get_layout() -> Option<String> {
  use std::process::Command;

  #[cfg(target_os = "linux")]
  {
    let xkbmap_query = Command::new("setxkbmap").arg("-query").output();
    debug!("xkbmap output: {xkbmap_query:?}");

    if let Ok(valid_query) = xkbmap_query {
      let stdout_str = match str::from_utf8(&valid_query.stdout) {
        Ok(o) => o,
        Err(e) => {
          warn!("xkbmap's output could not be parsed to utf8: {e:?}");
          return None;
        }
      };

      for line in stdout_str.lines() {
        if line.starts_with("layout:") {
          return line.split_whitespace().last().map(|x| x.to_owned());
        }
      }
    }
  }

  None
}
