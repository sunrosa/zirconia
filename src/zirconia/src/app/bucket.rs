use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
  app::{PressCount, RawProgramName},
  prelude::*,
};

/// Program-level bucket that stores event data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bucket {
  pub events: HashMap<rdev::Key, PressCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct BucketKind {
  pub program: RawProgramName,
  /// Keyboard layout, *includes* language, so English would be `en-qwerty`, and Russian would be `ru-йцукен`. Set to [`None`] if layout cannot be determined.
  pub layout: Option<String>,
}
impl BucketKind {
  pub fn new(program: RawProgramName) -> Self {
    Self { program, layout: None }
  }
}
