use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
  time::Duration,
};

use iced::{
  Element, Task,
  widget::{column, row, text},
};
use rdev::Key;
use x_win::WindowInfo;

use crate::listener;

pub struct App {
  key_occurrences: HashMap<String, HashMap<rdev::Key, u32>>,
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
      key_occurrences: Default::default(),
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
        for occurrence in key_occurrences {
          *self
            .key_occurrences
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
    column![text!("{:?}", self.key_occurrences)].into()
  }
}
