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

use crate::listener;

pub struct App {
  key_occurrences: HashMap<rdev::Key, u32>,
}
pub enum Message {
  KeyboardEvents(HashMap<rdev::Key, u32>),
}

impl App {
  pub fn boot() -> (Self, Task<Message>) {
    let app = Self {
      key_occurrences: Default::default(),
    };
    let task = Task::batch([listener::task(Duration::from_secs(10))]);

    (app, task)
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    use Message::*;

    match message {
      KeyboardEvents(occurrences) => {
        for occurrence in occurrences {
          *self.key_occurrences.entry(occurrence.0).or_default() += occurrence.1;
        }
      }
    }

    Task::none()
  }

  pub fn view<'a>(&'a self) -> Element<'a, Message> {
    column![text!("{:?}", self.key_occurrences)].into()
  }
}
