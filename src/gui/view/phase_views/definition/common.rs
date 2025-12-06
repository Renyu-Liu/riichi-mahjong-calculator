use crate::gui::messages::Message;
use iced::widget::{checkbox, row, text};
use iced::{Color, Element};

pub fn section_header(title: &str) -> Element<'_, Message> {
    text(title)
        .size(20)
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..iced::Font::with_name("Arimo")
        })
        .into()
}

pub fn checkbox_with_conflict<'a>(
    label: &str,
    is_checked: bool,
    msg: fn(bool) -> Message,
    is_enabled: bool,
) -> Element<'a, Message> {
    if is_enabled {
        checkbox(label, is_checked).on_toggle(msg).into()
    } else {
        row![
            checkbox("", is_checked),
            text(label).style(Color::from_rgb(0.5, 0.5, 0.5))
        ]
        .spacing(0)
        .align_items(iced::Alignment::Center)
        .into()
    }
}
