use super::common::section_header;
use crate::gui::components::action_button;
use crate::gui::messages::Message;
use crate::gui::state::RiichiGui;
use crate::gui::styles::ColoredButtonStyle;
use crate::implements::game::AgariType;
use crate::implements::tiles::Kaze;
use iced::widget::{button, column, radio, row, text};
use iced::{Color, Element, theme};

pub fn build_game_info_section(gui: &RiichiGui) -> Element<'_, Message> {
    column![
        section_header("Game Info"),
        row![
            text("Win Type:"),
            radio(
                "Ron",
                AgariType::Ron,
                Some(gui.agari_type),
                Message::ToggleAgariType
            ),
            radio(
                "Tsumo",
                AgariType::Tsumo,
                Some(gui.agari_type),
                Message::ToggleAgariType
            ),
        ]
        .spacing(20),
        row![
            text("Prevalent Wind:"),
            radio("East", Kaze::Ton, Some(gui.bakaze), Message::SetBakaze),
            radio("South", Kaze::Nan, Some(gui.bakaze), Message::SetBakaze),
            radio("West", Kaze::Shaa, Some(gui.bakaze), Message::SetBakaze),
            radio("North", Kaze::Pei, Some(gui.bakaze), Message::SetBakaze),
        ]
        .spacing(10),
        row![
            text("Seat Wind:"),
            radio("East", Kaze::Ton, Some(gui.jikaze), Message::SetJikaze),
            radio("South", Kaze::Nan, Some(gui.jikaze), Message::SetJikaze),
            radio("West", Kaze::Shaa, Some(gui.jikaze), Message::SetJikaze),
            radio("North", Kaze::Pei, Some(gui.jikaze), Message::SetJikaze),
        ]
        .spacing(10),
        row![
            text(format!("Honba: {}", gui.honba)),
            action_button("+", Message::IncrementHonba, ColoredButtonStyle::INFO,),
            button(text("-"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press_maybe(if gui.honba > 0 {
                    Some(Message::DecrementHonba)
                } else {
                    None
                }),
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center),
    ]
    .spacing(15)
    .align_items(iced::Alignment::Center)
    .into()
}
