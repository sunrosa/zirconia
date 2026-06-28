use crate::{heatmap::keyboard, prelude::*};

use std::{
  collections::{BTreeMap, HashMap, HashSet},
  sync::Arc,
  time::Duration,
};

use chrono::{DateTime, Timelike, Utc};
use iced::{
  Element, Length, Size, Task,
  alignment::Horizontal,
  futures::SinkExt,
  widget::{column, container, row, scrollable, text},
};
use kanal::{Receiver, Sender};
use rdev::Key;
use serde::{Deserialize, Serialize};
use x_win::WindowInfo;

use tokio::{io::AsyncWriteExt, task::JoinHandle};

use crate::listener;

#[derive(Debug)]
pub struct App {
  key_buckets: BTreeMap<DateTime<Utc>, BTreeMap<String, HashMap<rdev::Key, u32>>>,
}
impl App {
  fn as_persistent(&self) -> AppPersistent {
    AppPersistent {
      key_buckets: self.key_buckets.clone(),
    }
  }

  fn from_persistent(persistent: AppPersistent) -> Self {
    Self {
      key_buckets: persistent.key_buckets,
    }
  }
}

impl Default for App {
  fn default() -> Self {
    Self {
      key_buckets: Default::default(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Message {
  KeyboardEvents {
    key_occurrences: HashMap<rdev::Key, u32>,
    active_window: WindowInfo,
  },
  AutosaveNow,
  CloseApp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPersistent {
  key_buckets: BTreeMap<DateTime<Utc>, BTreeMap<String, HashMap<rdev::Key, u32>>>,
}

impl App {
  #[instrument(skip_all, level = Level::INFO)]
  pub fn boot() -> (Self, Task<Message>) {
    let app = if let Ok(file) = std::fs::File::open("save.ron") {
      App::from_persistent(ron::de::from_reader(file).unwrap())
    } else {
      Self::default()
    };

    let task = Task::batch([listener::task_run(Duration::from_secs(1)), autosave_signal()]);

    (app, task)
  }

  #[instrument(skip_all, level = Level::DEBUG)]
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
      AutosaveNow => return self.save(),
      CloseApp => return self.close(),
    }

    Task::none()
  }

  #[instrument(skip_all, level = Level::DEBUG)]
  pub fn view<'a>(&'a self) -> Element<'a, Message> {
    // A collection of all keypresses across all time buckets and programs
    let mut keypresses = HashMap::new();

    let mut formatted_text = String::with_capacity(512);
    for time_bucket in &self.key_buckets {
      formatted_text += &format!("{:?}\n", time_bucket.0);

      for program in time_bucket.1 {
        formatted_text += &format!("    {:?}\n", program.0);

        let mut sorted_keys: Vec<_> = program.1.into_iter().collect();
        sorted_keys.sort_by(|a, b| b.1.cmp(a.1));

        for key in sorted_keys {
          formatted_text += &format!("        {:?}: {}\n", key.0, key.1);

          *keypresses.entry(*key.0).or_default() += key.1;
        }
      }
    }

    column![
      container(keyboard(&keypresses).height(Length::Fill).width(Length::Shrink))
        .center_x(Length::Fill)
        .height(Length::Shrink),
      scrollable(text(formatted_text))
        .height(Length::Fill)
        .width(Length::Fill),
    ]
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
  }

  #[instrument(skip_all, level = Level::DEBUG)]
  fn save(&self) -> Task<Message> {
    let to_save = self.as_persistent();

    Task::future(async move {
      debug!("saving app");

      let mut writer = tokio::fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("save.ron")
        .await
        .unwrap();

      let buf = ron::ser::to_string(&to_save).unwrap();
      writer.write_all(buf.as_bytes()).await.unwrap();
    })
    .discard()
  }

  #[instrument(skip_all, level = Level::INFO)]
  fn close(&self) -> Task<Message> {
    info!("saving and closing app now!");

    // Close the listener thread (THIS DOESN'T WORK BECAUSE BLOCKING THREADS CAN'T BE ABORTED)
    // self.listener_handle.as_ref().map(|x| x.abort());

    // Then close the iced app and runtime
    self.save().chain(iced::exit())
  }
}

#[instrument(skip_all, level = Level::TRACE)]
fn autosave_signal() -> Task<Message> {
  Task::stream(iced::stream::channel(4, async move |mut output| {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
      output.send(Message::AutosaveNow).await.unwrap();
      interval.tick().await;
    }
  }))
}
