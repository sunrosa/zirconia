//! # TODO
//! - Graphs over time
//! - Network usage tracking
//! - Mouse clicks and position tracking to make a heatmap of the screen (OFF BY DEFAULT. a lot of people won't like it, since it allows for fairly precise fingerprinting of what they're doing)
//!
//! ## Integration
//! - Integrate with wakapi
//! - Integrate with MIDI keyboards
//! - Integrate with system keyboard layout detection, to record how users type in different languages and layouts

#![allow(unused_imports, dead_code)]

mod prelude;
use prelude::*;

use iced::{Subscription, window};

use tracing_subscriber::layer::SubscriberExt;

use crate::app::{App, Message};

mod app;
mod listener;
mod math;
mod newtype_macro;

#[instrument(skip_all, level = Level::INFO)]
fn main() {
  // tracing::subscriber::set_global_default(tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()))
  //   .expect("setting up tracy layer");

  // tracing_log::LogTracer::init().unwrap();

  iced::application(App::boot, App::update, App::view)
    .title("Zirconia")
    .window(window::Settings {
      fullscreen: false,
      exit_on_close_request: false,
      ..Default::default()
    })
    .subscription(subscriptions)
    .run()
    .unwrap();

  // In order to kill the fucking rdev::listen thread. This runs after the iced thread has already existed.
  // BUG Nevermind it doesn't work.
  std::process::exit(0);

  // If that doesn't fucking work (it doesn't on linux), just abort it. It's possible rdev is spawning child processes on linux (though `pgrep -P` says no).
  #[allow(unreachable_code)] // Even the lint is wrong.
  std::process::abort();
}

#[instrument(skip_all, level = Level::TRACE)]
fn subscriptions(state: &App) -> Subscription<Message> {
  Subscription::batch([
    window::close_requests().map(|id| {
      debug!("close request received to close window of ID {id:?}");
      Message::CloseApp
    }),
    window::close_events().map(|id| {
      debug!("close event received for window of ID {id:?}");
      Message::CloseApp
    }),
  ])
}
