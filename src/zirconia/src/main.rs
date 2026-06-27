//! # TODO
//! ## UI
//! - Make a keyboad heatmap
//!   - It shouldn't be that hard. I'll just use a column of 6 rows and fill those with weighted-width squares, and with centered text, whose background I can color for heatmapping. See ISO 9995. If you pay close attention, the keyboard keys are positioned in left-to-right steps such that each key in the main area occupies a unique horizontal position, such that the typewriter levers under them could go "up" (toward the fn keys) and each have their own area. It goes QA2ZWS3X... horizontally, with each following key being more to the right than the previous.
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
  // tracing::subsdriber::set_global_default(tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()))
  //   .expect("setting up tracy layer");

  iced::application(App::boot, App::update, App::view)
    .subscription(subscriptions)
    .run()
    .unwrap();
}

#[instrument(skip_all, level = Level::TRACE)]
fn subscriptions(state: &App) -> Subscription<Message> {
  Subscription::batch([
    window::close_requests().map(|_| Message::CloseApp),
    window::close_events().map(|_| Message::CloseApp),
  ])
}
