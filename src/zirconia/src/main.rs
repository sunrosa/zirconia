//! # TODO
//! - The scrollable is quite laggy when drawing the heatmap and scrolling. I profiled it, and it's mostly empty space (nothing running), so I don't know what's going on
//!
//! ## Integration
//! - Integrate with wakapi
//! - Integrate with MIDI keyboards

#![allow(unused_imports, dead_code)]

mod prelude;
use prelude::*;

use iced::{Subscription, window};

use tracing_subscriber::layer::SubscriberExt;

use crate::app::{App, Message};

mod app;
mod heatmap;
mod listener;

#[instrument(skip_all, level = Level::INFO)]
fn main() {
  tracing::subscriber::set_global_default(tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()))
    .expect("setting up tracy layer");

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
      Message::CloseApp}),
  ])
}
