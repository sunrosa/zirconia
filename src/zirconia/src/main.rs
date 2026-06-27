#![allow(unused_imports, dead_code)]

mod prelude;
use prelude::*;

use tracing_subscriber::layer::SubscriberExt;

use crate::app::App;

mod app;
mod listener;

#[instrument(skip_all, level = Level::INFO)]
fn main() {
  tracing::subscriber::set_global_default(tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()))
    .expect("setting up tracy layer");

  iced::application(App::boot, App::update, App::view).run().unwrap();
}
