use std::collections::HashMap;

use iced::{
  Background, Border, Color, Element, Length, Padding,
  alignment::Vertical,
  widget::{
    Container, Responsive, container,
    grid::{self, Sizing},
    responsive, text,
  },
};

use crate::{app::PressCount, math::sigmoid, prelude::*};

use iced::widget::{column, row};

static KEY_SPACING: f32 = 1.;

#[instrument(skip_all, level = Level::DEBUG)]
pub fn keyboard<'a>(key_data: &'_ HashMap<rdev::Key, PressCount>) -> Responsive<'a, Message> {
  use Length::*;
  use rdev::Key::*;

  let total_keypresses = key_data.iter().fold(PressCount::default(), |a, data| a + *data.1);

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
          *key_data.get(&Escape).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F1",
          FillPortion(100),
          *key_data.get(&F1).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F2",
          FillPortion(100),
          *key_data.get(&F2).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F3",
          FillPortion(100),
          *key_data.get(&F3).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F4",
          FillPortion(100),
          *key_data.get(&F4).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F5",
          FillPortion(100),
          *key_data.get(&F5).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F6",
          FillPortion(100),
          *key_data.get(&F6).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F7",
          FillPortion(100),
          *key_data.get(&F7).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F8",
          FillPortion(100),
          *key_data.get(&F8).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F9",
          FillPortion(100),
          *key_data.get(&F9).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F10",
          FillPortion(100),
          *key_data.get(&F10).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F11",
          FillPortion(100),
          *key_data.get(&F11).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F12",
          FillPortion(100),
          *key_data.get(&F12).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Home",
          FillPortion(100),
          *key_data.get(&Home).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "End",
          FillPortion(100),
          *key_data.get(&End).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Insert",
          FillPortion(100),
          *key_data.get(&Insert).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Delete",
          FillPortion(150),
          *key_data.get(&Delete).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
      ]
      .height(FillPortion(60)),
      row![
        // 1500 total
        keycap(
          "`",
          FillPortion(100),
          *key_data.get(&BackQuote).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "1",
          FillPortion(100),
          *key_data.get(&Num1).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "2",
          FillPortion(100),
          *key_data.get(&Num2).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "3",
          FillPortion(100),
          *key_data.get(&Num3).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "4",
          FillPortion(100),
          *key_data.get(&Num4).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "5",
          FillPortion(100),
          *key_data.get(&Num5).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "6",
          FillPortion(100),
          *key_data.get(&Num6).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "7",
          FillPortion(100),
          *key_data.get(&Num7).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "8",
          FillPortion(100),
          *key_data.get(&Num8).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "9",
          FillPortion(100),
          *key_data.get(&Num9).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "0",
          FillPortion(100),
          *key_data.get(&Num0).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "-",
          FillPortion(100),
          *key_data.get(&Minus).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "=",
          FillPortion(100),
          *key_data.get(&Equal).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Backspace",
          FillPortion(200),
          *key_data.get(&Backspace).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
      ]
      .height(FillPortion(100)),
      row![
        // 1500 total
        keycap(
          "Tab",
          FillPortion(150),
          *key_data.get(&Tab).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Q",
          FillPortion(100),
          *key_data.get(&KeyQ).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "W",
          FillPortion(100),
          *key_data.get(&KeyW).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "E",
          FillPortion(100),
          *key_data.get(&KeyE).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "R",
          FillPortion(100),
          *key_data.get(&KeyR).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "T",
          FillPortion(100),
          *key_data.get(&KeyT).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Y",
          FillPortion(100),
          *key_data.get(&KeyY).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "U",
          FillPortion(100),
          *key_data.get(&KeyU).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "I",
          FillPortion(100),
          *key_data.get(&KeyI).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "O",
          FillPortion(100),
          *key_data.get(&KeyO).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "P",
          FillPortion(100),
          *key_data.get(&KeyP).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "[",
          FillPortion(100),
          *key_data.get(&LeftBracket).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "]",
          FillPortion(100),
          *key_data.get(&RightBracket).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "\\",
          FillPortion(150),
          *key_data.get(&BackSlash).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
      ]
      .height(FillPortion(100)),
      row![
        // 1500 total
        keycap(
          "CapsLock",
          FillPortion(175),
          *key_data.get(&CapsLock).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "A",
          FillPortion(100),
          *key_data.get(&KeyA).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "S",
          FillPortion(100),
          *key_data.get(&KeyS).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "D",
          FillPortion(100),
          *key_data.get(&KeyD).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "F",
          FillPortion(100),
          *key_data.get(&KeyF).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "G",
          FillPortion(100),
          *key_data.get(&KeyG).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "H",
          FillPortion(100),
          *key_data.get(&KeyH).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "J",
          FillPortion(100),
          *key_data.get(&KeyJ).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "K",
          FillPortion(100),
          *key_data.get(&KeyK).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "L",
          FillPortion(100),
          *key_data.get(&KeyL).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          ";",
          FillPortion(100),
          *key_data.get(&SemiColon).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "'",
          FillPortion(100),
          *key_data.get(&Quote).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Enter",
          FillPortion(225),
          *key_data.get(&Return).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
      ]
      .height(FillPortion(100)),
      row![
        // 1500 total
        keycap(
          "Shift",
          FillPortion(225),
          *key_data.get(&ShiftLeft).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Z",
          FillPortion(100),
          *key_data.get(&KeyZ).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "X",
          FillPortion(100),
          *key_data.get(&KeyX).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "C",
          FillPortion(100),
          *key_data.get(&KeyC).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "V",
          FillPortion(100),
          *key_data.get(&KeyV).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "B",
          FillPortion(100),
          *key_data.get(&KeyB).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "N",
          FillPortion(100),
          *key_data.get(&KeyN).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "M",
          FillPortion(100),
          *key_data.get(&KeyM).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          ",",
          FillPortion(100),
          *key_data.get(&Comma).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          ".",
          FillPortion(100),
          *key_data.get(&Dot).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "/",
          FillPortion(100),
          *key_data.get(&Slash).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Shift",
          FillPortion(275),
          *key_data.get(&ShiftRight).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
      ]
      .height(FillPortion(100)),
      row![
        keycap(
          "Ctrl",
          FillPortion(225),
          *key_data.get(&ControlLeft).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Meta",
          FillPortion(100),
          *key_data.get(&MetaLeft).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Alt",
          FillPortion(100),
          *key_data.get(&Alt).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Space",
          FillPortion(503),
          *key_data.get(&Space).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Alt",
          FillPortion(100),
          *key_data.get(&AltGr).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "Ctrl",
          FillPortion(100),
          *key_data.get(&ControlRight).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "←",
          FillPortion(93),
          *key_data.get(&LeftArrow).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "↓",
          FillPortion(93),
          *key_data.get(&DownArrow).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "↑",
          FillPortion(93),
          *key_data.get(&UpArrow).unwrap_or(&PressCount::default()),
          total_keypresses
        ),
        keycap(
          "→",
          FillPortion(93),
          *key_data.get(&RightArrow).unwrap_or(&PressCount::default()),
          total_keypresses
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

#[instrument(skip_all, level = Level::TRACE)]
fn keycap<'a>(
  label: &'a str,
  width: Length,
  times_pressed: PressCount,
  total_pressed: PressCount,
) -> Container<'a, Message> {
  let press_percentage = times_pressed.0 as f32 / total_pressed.0 as f32;

  // From 0 to 1
  // WARN The number of keys matters here. If more keys are added, this needs to be updated
  let hotness = sigmoid(press_percentage * 100. - 1.8);

  // TODO use theming here, and use oklab via the palette crate.
  let background_color = Color {
    a: 1.,
    r: hotness * 0.8 + 0.2,
    g: (hotness / 8.) * 0.8 + 0.2,
    b: (hotness / 3.) * 0.8 + 0.2,
  };

  // The reason we have a container inside a container is so we can use PADDING instead of row spacing, which occurs on the *inside*. That means the key sizes are exact, and rows with more keys don't desync with rows with less keys.
  container(
    container(column![
      text(label)
        .align_x(text::Alignment::Center)
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill),
      text(times_pressed.0)
        .align_x(text::Alignment::Center)
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
    ])
    .style(move |theme| {
      let default_style = container::rounded_box(theme);
      container::Style {
        border: Border {
          width: 1.,
          radius: 5.0.into(),
          ..default_style.border
        },
        background: Some(Background::Color(background_color)),
        ..default_style
      }
    }),
  )
  .width(width)
  .height(Length::Fill)
  .padding(Padding {
    left: KEY_SPACING / 2.,
    right: KEY_SPACING / 2.,
    top: 0.,
    bottom: 0.,
  })
  .into()
}
