use super::common::section_header;
use crate::gui::components::{action_button, tile_image_button};
use crate::gui::messages::Message;
use crate::gui::state::RiichiGui;
use crate::gui::styles::ColoredButtonStyle;
use iced::widget::{column, row};
use iced::{Element, theme};

pub fn build_winning_tile_section(gui: &RiichiGui) -> Element<'_, Message> {
    column![
        section_header("Winning Tile"),
        match &gui.winning_tile {
            Some(t) => {
                let handle = gui
                    .tile_images
                    .get(t)
                    .expect("Tile image not found")
                    .clone();
                let e: Element<Message> = row![tile_image_button(
                    handle,
                    40,
                    Message::StartSelectWinningTile,
                    theme::Button::Custom(Box::new(ColoredButtonStyle::NEUTRAL_HOVER)),
                )]
                .spacing(10)
                .align_items(iced::Alignment::Center)
                .into();
                e
            }
            None => row![action_button(
                "Select",
                Message::StartSelectWinningTile,
                ColoredButtonStyle::INFO,
            )]
            .align_items(iced::Alignment::Center)
            .into(),
        }
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .into()
}
