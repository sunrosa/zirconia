use std::collections::HashMap;

use iced::{
  Border, Element, Length, Padding,
  alignment::Vertical,
  widget::{
    Container, container,
    grid::{self, Sizing},
    responsive, text,
  },
};

use crate::prelude::*;

use iced::widget::{column, row};

static KEY_SPACING: f32 = 2.;

pub fn keyboard<'a>(key_data: &'_ HashMap<rdev::Key, u32>) -> Element<'a, Message> {
  use Length::*;
  use rdev::Key::*;

  let total_keypresses = key_data.iter().fold(0, |a, data| a + data.1);

  let key_data = key_data.clone();

  responsive(move |size| {
    let aspect_ratio = 2.8 / 1.0;

    let new_size = size.ratio(aspect_ratio);

    column![
      row![
        // Total irrelevant
        keycap(
          "Esc",
          FillPortion(150),
          *key_data.get(&Escape).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F1",
          FillPortion(100),
          *key_data.get(&F1).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F2",
          FillPortion(100),
          *key_data.get(&F2).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F3",
          FillPortion(100),
          *key_data.get(&F3).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F4",
          FillPortion(100),
          *key_data.get(&F4).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F5",
          FillPortion(100),
          *key_data.get(&F5).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F6",
          FillPortion(100),
          *key_data.get(&F6).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F7",
          FillPortion(100),
          *key_data.get(&F7).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F8",
          FillPortion(100),
          *key_data.get(&F8).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F9",
          FillPortion(100),
          *key_data.get(&F9).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F10",
          FillPortion(100),
          *key_data.get(&F10).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F11",
          FillPortion(100),
          *key_data.get(&F11).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F12",
          FillPortion(100),
          *key_data.get(&F12).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Home",
          FillPortion(100),
          *key_data.get(&Home).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "End",
          FillPortion(100),
          *key_data.get(&End).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Insert",
          FillPortion(100),
          *key_data.get(&Insert).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Delete",
          FillPortion(150),
          *key_data.get(&Delete).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
      ]
      .height(FillPortion(60)),
      row![
        // 1500 total
        keycap(
          "`",
          FillPortion(100),
          *key_data.get(&BackQuote).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "1",
          FillPortion(100),
          *key_data.get(&Num1).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "2",
          FillPortion(100),
          *key_data.get(&Num2).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "3",
          FillPortion(100),
          *key_data.get(&Num3).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "4",
          FillPortion(100),
          *key_data.get(&Num4).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "5",
          FillPortion(100),
          *key_data.get(&Num5).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "6",
          FillPortion(100),
          *key_data.get(&Num6).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "7",
          FillPortion(100),
          *key_data.get(&Num7).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "8",
          FillPortion(100),
          *key_data.get(&Num8).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "9",
          FillPortion(100),
          *key_data.get(&Num9).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "0",
          FillPortion(100),
          *key_data.get(&Num0).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "-",
          FillPortion(100),
          *key_data.get(&Minus).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "=",
          FillPortion(100),
          *key_data.get(&Equal).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Backspace",
          FillPortion(200),
          *key_data.get(&Backspace).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
      ]
      .height(FillPortion(100)),
      row![
        // 1500 total
        keycap(
          "Tab",
          FillPortion(150),
          *key_data.get(&Tab).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Q",
          FillPortion(100),
          *key_data.get(&KeyQ).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "W",
          FillPortion(100),
          *key_data.get(&KeyW).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "E",
          FillPortion(100),
          *key_data.get(&KeyE).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "R",
          FillPortion(100),
          *key_data.get(&KeyR).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "T",
          FillPortion(100),
          *key_data.get(&KeyT).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Y",
          FillPortion(100),
          *key_data.get(&KeyY).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "U",
          FillPortion(100),
          *key_data.get(&KeyU).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "I",
          FillPortion(100),
          *key_data.get(&KeyI).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "O",
          FillPortion(100),
          *key_data.get(&KeyO).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "P",
          FillPortion(100),
          *key_data.get(&KeyP).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "[",
          FillPortion(100),
          *key_data.get(&LeftBracket).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "]",
          FillPortion(100),
          *key_data.get(&RightBracket).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "\\",
          FillPortion(150),
          *key_data.get(&BackSlash).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
      ]
      .height(FillPortion(100)),
      row![
        // 1500 total
        keycap(
          "CapsLock",
          FillPortion(175),
          *key_data.get(&CapsLock).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "A",
          FillPortion(100),
          *key_data.get(&KeyA).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "S",
          FillPortion(100),
          *key_data.get(&KeyS).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "D",
          FillPortion(100),
          *key_data.get(&KeyD).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "F",
          FillPortion(100),
          *key_data.get(&KeyF).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "G",
          FillPortion(100),
          *key_data.get(&KeyG).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "H",
          FillPortion(100),
          *key_data.get(&KeyH).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "J",
          FillPortion(100),
          *key_data.get(&KeyJ).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "K",
          FillPortion(100),
          *key_data.get(&KeyK).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "L",
          FillPortion(100),
          *key_data.get(&KeyL).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          ";",
          FillPortion(100),
          *key_data.get(&SemiColon).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "'",
          FillPortion(100),
          *key_data.get(&Quote).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Enter",
          FillPortion(225),
          *key_data.get(&Return).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
      ]
      .height(FillPortion(100)),
      row![
        // 1500 total
        keycap(
          "Shift",
          FillPortion(225),
          *key_data.get(&ShiftLeft).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Z",
          FillPortion(100),
          *key_data.get(&KeyZ).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "X",
          FillPortion(100),
          *key_data.get(&KeyX).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "C",
          FillPortion(100),
          *key_data.get(&KeyC).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "V",
          FillPortion(100),
          *key_data.get(&KeyV).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "B",
          FillPortion(100),
          *key_data.get(&KeyB).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "N",
          FillPortion(100),
          *key_data.get(&KeyN).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "M",
          FillPortion(100),
          *key_data.get(&KeyM).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          ",",
          FillPortion(100),
          *key_data.get(&Comma).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          ".",
          FillPortion(100),
          *key_data.get(&Dot).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "/",
          FillPortion(100),
          *key_data.get(&Slash).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Shift",
          FillPortion(275),
          *key_data.get(&ShiftRight).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
      ]
      .height(FillPortion(100)),
      row![
        keycap(
          "Ctrl",
          FillPortion(225),
          *key_data.get(&ControlLeft).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Meta",
          FillPortion(100),
          *key_data.get(&MetaLeft).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Alt",
          FillPortion(100),
          *key_data.get(&Alt).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Space",
          FillPortion(503),
          *key_data.get(&Space).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Alt",
          FillPortion(100),
          *key_data.get(&AltGr).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "Ctrl",
          FillPortion(100),
          *key_data.get(&ControlRight).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "←",
          FillPortion(93),
          *key_data.get(&LeftArrow).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "↓",
          FillPortion(93),
          *key_data.get(&DownArrow).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "↑",
          FillPortion(93),
          *key_data.get(&UpArrow).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
        keycap(
          "→",
          FillPortion(93),
          *key_data.get(&RightArrow).unwrap_or(&0) as f64 / total_keypresses as f64
        ),
      ]
      .height(FillPortion(100)),
    ]
    .width(Fixed(new_size.width))
    .height(Fixed(new_size.height))
    .spacing(KEY_SPACING)
  })
  .into()
}

fn keycap<'a>(label: &'a str, width: Length, percentage: f64) -> Container<'a, Message> {
  // The reason we have a container inside a container is so we can use PADDING instead of row spacing, which occurs on the *inside*. That means the key sizes are exact, and rows with more keys don't desync with rows with less keys.
  container(
    container(
      text(label)
        .align_x(text::Alignment::Center)
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .style(move |theme| {
      let default_style = container::rounded_box(theme);
      container::Style {
        border: Border {
          width: 3.,
          radius: 5.0.into(),
          ..default_style.border
        },
        // background: Some(Background::Color(Color { a: 0.1, ..text_color })),
        ..default_style
      }
    }),
  )
  .width(width)
  .height(Length::Fill)
  .padding(Padding {
    left: KEY_SPACING / 2.,
    right: KEY_SPACING / 2.,
    ..Default::default()
  })
  .into()
}
