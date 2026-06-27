use std::{
  collections::{BTreeMap, HashMap, HashSet},
  sync::Arc,
  time::Duration,
};

use chrono::{DateTime, Timelike, Utc};
use iced::{
  Element, Length, Task,
  widget::{column, row, scrollable, text},
};
use rdev::Key;
use x_win::WindowInfo;

use crate::listener;

pub struct App {
  key_buckets: BTreeMap<DateTime<Utc>, BTreeMap<String, HashMap<rdev::Key, u32>>>,
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
    let task = Task::batch([listener::task_run(Duration::from_secs(2))]);

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
            .entry(format!(
              "{} - {}",
              active_window.info.exec_name.clone(),
              active_window.info.name.clone()
            ))
            .or_default()
            .entry(occurrence.0)
            .or_default() += occurrence.1;
        }
      }
    }

    Task::none()
  }

  pub fn view<'a>(&'a self) -> Element<'a, Message> {
    let mut formatted_text = String::with_capacity(512);

    for time_bucket in &self.key_buckets {
      formatted_text += &format!("{:?}\n", time_bucket.0);

      for program in time_bucket.1 {
        formatted_text += &format!("  {:?}\n", program.0);

        let mut sorted_keys: Vec<_> = program.1.into_iter().collect();
        sorted_keys.sort_by(|a, b| b.1.cmp(a.1));

        for key in sorted_keys {
          formatted_text += &format!("    {:?}: {}\n", key.0, key.1);
        }
      }
    }

    scrollable(column![text(formatted_text)].width(Length::Fill)).into()
  }
}
