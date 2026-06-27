#![allow(unused_imports, dead_code)]

mod prelude;

use crate::app::App;

mod app;
mod listener;

fn main() {
  iced::application(App::boot, App::update, App::view).run().unwrap();
}
