#![allow(unused_imports, dead_code)]

mod prelude;
use iced::{Subscription, window};
use prelude::*;

use tracing_subscriber::layer::SubscriberExt;

use crate::app::{App, Message};

mod app;
mod listener;

#[instrument(skip_all, level = Level::INFO)]
fn main() {
  // tracing::subsdriber::set_global_default(tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()))
  //   .expect("setting up tracy layer");

  iced::application(App::boot, App::update, App::view).subscription(subscriptions).run().unwrap();
}

fn subscriptions(state: &App) -> Subscription<Message> {
  Subscription::batch([
    window::close_requests().map(|_| Message::CloseApp)
  ])
}
