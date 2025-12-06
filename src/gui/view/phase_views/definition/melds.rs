use super::common::section_header;
use crate::gui::components::{action_button, tile_image};
use crate::gui::messages::Message;
use crate::gui::state::RiichiGui;
use crate::gui::styles::ColoredButtonStyle;
use crate::implements::hand::MentsuType;
use iced::widget::{button, column, row};
use iced::{Element, theme};

pub fn build_melds_section(gui: &RiichiGui) -> Element<'_, Message> {
    column![
        section_header("Open Melds"),
        // Existing open melds
        column(
            gui.open_melds
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let tiles = gui.get_meld_tiles(m);
                    let tile_images = row(tiles
                        .iter()
                        .map(|t| {
                            let handle = gui
                                .tile_images
                                .get(t)
                                .expect("Tile image not found")
                                .clone();
                            tile_image(handle, 40)
                        })
                        .collect::<Vec<Element<Message>>>())
                    .spacing(2);

                    row![
                        button(tile_images)
                            .on_press(Message::RemoveOpenMeld(i))
                            .style(theme::Button::Text)
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .into()
                })
                .collect::<Vec<Element<Message>>>()
        )
        .align_items(iced::Alignment::Center)
        .spacing(10),
        // Existing closed kans
        column(
            gui.closed_kans
                .iter()
                .enumerate()
                .map(|(i, k)| {
                    let tiles = vec![*k; 4];
                    let tile_images = row(tiles
                        .iter()
                        .map(|t| {
                            let handle = gui
                                .tile_images
                                .get(t)
                                .expect("Tile image not found")
                                .clone();
                            tile_image(handle, 40)
                        })
                        .collect::<Vec<Element<Message>>>())
                    .spacing(2);
                    row![
                        button(tile_images)
                            .on_press(Message::RemoveClosedKan(i))
                            .style(theme::Button::Text)
                    ]
                    .align_items(iced::Alignment::Center)
                    .into()
                })
                .collect::<Vec<Element<Message>>>()
        )
        .spacing(10),
        // Add meld buttons
        row![
            action_button(
                "Add Pon",
                Message::SelectMeldType(MentsuType::Koutsu),
                ColoredButtonStyle::INFO,
            ),
            action_button(
                "Add Chii",
                Message::SelectMeldType(MentsuType::Shuntsu),
                ColoredButtonStyle::INFO,
            ),
            action_button(
                "Add Kan",
                Message::StartAddClosedKan,
                ColoredButtonStyle::INFO,
            )
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center)
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .into()
}
