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

pub fn keyboard<'a>() -> Element<'a, Message> {
  use Length::*;

  responsive(|size| {
    let aspect_ratio = 2.8 / 1.0;

    let new_size = size.ratio(aspect_ratio);

    column![
      row![
        // Total irrelevant
        keycap("Esc", FillPortion(150)),
        keycap("F1", FillPortion(100)),
        keycap("F2", FillPortion(100)),
        keycap("F3", FillPortion(100)),
        keycap("F4", FillPortion(100)),
        keycap("F5", FillPortion(100)),
        keycap("F6", FillPortion(100)),
        keycap("F7", FillPortion(100)),
        keycap("F8", FillPortion(100)),
        keycap("F9", FillPortion(100)),
        keycap("F10", FillPortion(100)),
        keycap("F11", FillPortion(100)),
        keycap("F12", FillPortion(100)),
        keycap("Home", FillPortion(100)),
        keycap("End", FillPortion(100)),
        keycap("Insert", FillPortion(100)),
        keycap("Delete", FillPortion(150)),
      ]
      .height(FillPortion(60)),
      row![
        // 1500 total
        keycap("`", FillPortion(100)),
        keycap("1", FillPortion(100)),
        keycap("2", FillPortion(100)),
        keycap("3", FillPortion(100)),
        keycap("4", FillPortion(100)),
        keycap("5", FillPortion(100)),
        keycap("6", FillPortion(100)),
        keycap("7", FillPortion(100)),
        keycap("8", FillPortion(100)),
        keycap("9", FillPortion(100)),
        keycap("0", FillPortion(100)),
        keycap("-", FillPortion(100)),
        keycap("=", FillPortion(100)),
        keycap("Backspace", FillPortion(200)),
      ]
      .height(FillPortion(100)),
      row![
        // 1500 total
        keycap("Tab", FillPortion(150)),
        keycap("Q", FillPortion(100)),
        keycap("W", FillPortion(100)),
        keycap("E", FillPortion(100)),
        keycap("R", FillPortion(100)),
        keycap("T", FillPortion(100)),
        keycap("Y", FillPortion(100)),
        keycap("U", FillPortion(100)),
        keycap("I", FillPortion(100)),
        keycap("O", FillPortion(100)),
        keycap("P", FillPortion(100)),
        keycap("[", FillPortion(100)),
        keycap("]", FillPortion(100)),
        keycap("\\", FillPortion(150)),
      ]
      .height(FillPortion(100)),
      row![
        // 1500 total
        keycap("CapsLock", FillPortion(175)),
        keycap("A", FillPortion(100)),
        keycap("S", FillPortion(100)),
        keycap("D", FillPortion(100)),
        keycap("F", FillPortion(100)),
        keycap("G", FillPortion(100)),
        keycap("H", FillPortion(100)),
        keycap("J", FillPortion(100)),
        keycap("K", FillPortion(100)),
        keycap("L", FillPortion(100)),
        keycap(";", FillPortion(100)),
        keycap("'", FillPortion(100)),
        keycap("Enter", FillPortion(225)),
      ]
      .height(FillPortion(100)),
      row![
        // 1500 total
        keycap("Shift", FillPortion(225)),
        keycap("Z", FillPortion(100)),
        keycap("X", FillPortion(100)),
        keycap("C", FillPortion(100)),
        keycap("V", FillPortion(100)),
        keycap("B", FillPortion(100)),
        keycap("N", FillPortion(100)),
        keycap("M", FillPortion(100)),
        keycap(",", FillPortion(100)),
        keycap(".", FillPortion(100)),
        keycap("/", FillPortion(100)),
        keycap("Shift", FillPortion(275)),
      ]
      .height(FillPortion(100)),
      row![
        keycap("Ctrl", FillPortion(225)),
        keycap("Meta", FillPortion(100)),
        keycap("Alt", FillPortion(100)),
        keycap("Space", FillPortion(503)),
        keycap("Alt", FillPortion(100)),
        keycap("Ctrl", FillPortion(100)),
        keycap("←", FillPortion(93)),
        keycap("↓", FillPortion(93)),
        keycap("↑", FillPortion(93)),
        keycap("→", FillPortion(93)),
      ]
      .height(FillPortion(100)),
    ]
    .width(Fixed(new_size.width))
    .height(Fixed(new_size.height))
    .spacing(KEY_SPACING)
  })
  .into()
}

fn keycap<'a>(label: &'a str, width: Length) -> Container<'a, Message> {
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
