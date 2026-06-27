use std::{
  collections::{BTreeMap, HashMap, HashSet},
  sync::Arc,
  time::Duration,
};

use chrono::{DateTime, Timelike, Utc};
use iced::{
  Element, Task,
  widget::{column, row, text},
};
use rdev::Key;
use x_win::WindowInfo;

use crate::listener;

pub struct App {
  key_buckets: BTreeMap<DateTime<Utc>, HashMap<String, HashMap<rdev::Key, u32>>>,
}
pub enum Message {
  KeyboardEvents {
    key_occurrences: HashMap<rdev::Key, u32>,
    active_window: WindowInfo,
  },
}

impl App {
  pub fn boot() -> (Self, Task<Message>) {
    let app = Self {
      key_buckets: Default::default(),
    };
    let task = Task::batch([listener::task(Duration::from_secs(2))]);

    (app, task)
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    use Message::*;

    match message {
      KeyboardEvents {
        key_occurrences,
        active_window,
      } => {
        let time_now = chrono::Utc::now()
          .with_minute(0)
          .unwrap()
          .with_second(0)
          .unwrap()
          .with_nanosecond(0)
          .unwrap();

        for occurrence in key_occurrences {
          *self
            .key_buckets
            .entry(time_now)
            .or_default()
            .entry(active_window.info.exec_name.clone())
            .or_default()
            .entry(occurrence.0)
            .or_default() += occurrence.1;
        }
      }
    }

    Task::none()
  }

  pub fn view<'a>(&'a self) -> Element<'a, Message> {
    column![text!("{:?}", self.key_buckets)].into()
  }
}
