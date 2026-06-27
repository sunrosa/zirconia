use std::sync::Arc;

use iced::{Element, Task, widget::row};

use crate::listener;

pub struct App {}
pub enum Message {
  KeyboardEvents(Vec<rdev::Event>),
}

impl App {
  pub fn boot() -> (Self, Task<Message>) {
    let app = Self {};

    let mut task = Task::batch([listener::task()]);

    (app, task)
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    Task::none()
  }

  pub fn view<'a>(&'a self) -> Element<'a, Message> {
    row![].into()
  }
}
