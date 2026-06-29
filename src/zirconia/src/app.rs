use crate::app::bucket::{Bucket, BucketKind};
use crate::listener;
use crate::newtype_macro::{integer_newtypes, string_newtypes};
use crate::{app::heatmap::keyboard, prelude::*};
use chrono::{DateTime, Timelike, Utc};
use iced::message::MaybeClone;
use iced::{
  Element, Length, Size, Task,
  alignment::Horizontal,
  futures::SinkExt,
  widget::{column, container, row, scrollable, text},
};
use kanal::{Receiver, Sender};
use rdev::Key;
use serde::{Deserialize, Serialize};
use std::thread::JoinHandle;
use std::{
  collections::{BTreeMap, HashMap, HashSet},
  sync::Arc,
  time::Duration,
};
use tokio::io::AsyncWriteExt;
use tokio::time::MissedTickBehavior;
use x_win::WindowInfo;

mod bucket;
mod heatmap;

string_newtypes![
  /// (See its counterpart [`ProgramDisplayName`])
  ///
  /// The name of a program derived from system-level data.
  RawProgramName,

  /// (See its counterpart [`RawProgramName`])
  ///
  /// The display name the *user* has chosen for a program, which maps from a [`RawProgramName`]. Notably, users are allowed to merge programs by setting the same [`ProgramDisplayName`] for a corresponding [`RawProgramName`].
  ProgramDisplayName,
];

integer_newtypes![
  /// How many times a key has been pressed
  PressCount(u32)
];

/// Follows SemVer! Since it's a schema, it starts out at `1.0.0` because breaking changes immediately matter, and migrations will need to be written.
static SCHEMA_VERSION: &'static str = "1.0.0";

#[derive(Debug)]
pub struct App {
  /// Keypress data
  key_buckets: BTreeMap<DateTime<Utc>, HashMap<BucketKind, Bucket>>,

  /// The key is the raw program name, as stored in [`Self::key_buckets`]. The value is the display name the *user* has chosen to represent that program. If multiple programs map to an identical display name, they are treated as the same program in data analysis.
  program_names: HashMap<RawProgramName, ProgramDisplayName>,

  /// Critical threads that must be monitored
  critical_threads: Vec<Option<JoinHandle<()>>>,
}
impl App {
  fn as_persistent(&self) -> AppPersistent {
    AppPersistent {
      schema_version: SCHEMA_VERSION.to_string(),
      key_buckets: self.key_buckets.clone(),
      program_names: self.program_names.clone(),
    }
  }

  fn from_persistent(persistent: AppPersistent) -> Self {
    Self {
      key_buckets: persistent.key_buckets,
      program_names: persistent.program_names,
      ..Default::default()
    }
  }
}

impl Default for App {
  fn default() -> Self {
    Self {
      key_buckets: Default::default(),
      program_names: Default::default(),
      critical_threads: Default::default(),
    }
  }
}

#[derive(Debug)]
pub enum Message {
  KeyboardEvents {
    key_occurrences: HashMap<rdev::Key, PressCount>,
    active_window: WindowInfo,
  },
  AutosaveNow,
  CloseApp,

  /// A new critical thread was spawned that must be monitored
  NewCriticalThread(JoinHandle<()>),

  /// Check the state of all critical threads now, and handle it if some have exited.
  CheckCriticalThreads,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPersistent {
  schema_version: String,
  key_buckets: BTreeMap<DateTime<Utc>, HashMap<BucketKind, Bucket>>,
  program_names: HashMap<RawProgramName, ProgramDisplayName>,
}

impl App {
  #[instrument(skip_all, level = Level::INFO)]
  pub fn boot() -> (Self, Task<Message>) {
    let app = if let Ok(file) = std::fs::File::open("save.ron") {
      App::from_persistent(ron::de::from_reader(file).unwrap())
    } else {
      Self::default()
    };

    let task = Task::batch([
      listener::task_run(Duration::from_secs(1)),
      autosave_signal(),
      monitor_critical_threads(),
    ]);

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
            .entry(BucketKind::new(RawProgramName(format!(
              "{} - {}",
              active_window.info.exec_name.clone(),
              active_window.info.name.clone()
            ))))
            .or_default()
            .events
            .entry(occurrence.0)
            .or_default() += occurrence.1;
        }
      }
      AutosaveNow => return self.save(),
      CloseApp => return self.close(),
      NewCriticalThread(new_thread_handle) => self.critical_threads.push(Some(new_thread_handle)),
      CheckCriticalThreads => {
        for critical_thread in &mut self.critical_threads {
          if critical_thread.as_ref().is_some_and(|x| x.is_finished()) {
            let result = critical_thread.take().unwrap().join();
            error!("critical thread stopped: {result:?}");
            eprintln!("critical thread stopped: {result:?}");
            return self.close();
          }
        }
      }
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

        let mut sorted_keys: Vec<_> = program.1.events.iter().collect();
        sorted_keys.sort_by(|a, b| b.1.cmp(a.1));

        for key in sorted_keys {
          formatted_text += &format!("        {:?}: {}\n", key.0, key.1);

          *keypresses.entry(*key.0).or_default() += *key.1;
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

  /// Returns all programs seen by the key tracker
  fn known_bucketkinds(&self) -> HashSet<&BucketKind> {
    let mut programs = HashSet::new();

    for time_bucket in &self.key_buckets {
      for program_bucket in time_bucket.1 {
        programs.insert(program_bucket.0);
      }
    }

    programs
  }
}

#[instrument(skip_all, level = Level::TRACE)]
fn monitor_critical_threads() -> Task<Message> {
  Task::stream(iced::stream::channel(4, async move |mut output| {
    let mut interval = tokio::time::interval(Duration::from_secs(15));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
      output.send(Message::CheckCriticalThreads).await.unwrap();
      interval.tick().await;
    }
  }))
}

#[instrument(skip_all, level = Level::TRACE)]
fn autosave_signal() -> Task<Message> {
  Task::stream(iced::stream::channel(4, async move |mut output| {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
      output.send(Message::AutosaveNow).await.unwrap();
      interval.tick().await;
    }
  }))
}
