use super::super::super::components::{cancel_button, create_grid, tile_button};
use super::super::super::messages::Message;
use super::super::super::state::RiichiGui;
use super::super::super::styles::ColoredButtonStyle;
use crate::implements::tiles::Hai;
use iced::Element;
use iced::widget::{column, text};

impl RiichiGui {
    pub fn view_selecting_winning_tile(&self) -> Element<'_, Message> {
        let hand_tiles = self.hand_tiles.clone();
        iced::widget::lazy(hand_tiles, |hand_tiles: &Vec<Hai>| {
            let mut unique_tiles: Vec<Hai> = hand_tiles.clone();
            unique_tiles.sort();
            unique_tiles.dedup();

            let tiles: Vec<Element<Message>> = unique_tiles
                .iter()
                .map(|tile| {
                    let handle = self
                        .tile_images
                        .get(tile)
                        .expect("Tile image not found")
                        .clone();
                    tile_button(
                        iced::widget::Image::new(handle).width(50).into(),
                        Message::SelectWinningTile(*tile),
                        ColoredButtonStyle::NEUTRAL_HOVER,
                    )
                })
                .collect();

            let content: Element<'static, Message> = column![
                text("Select Winning Tile").size(24),
                create_grid(tiles, 10),
                cancel_button()
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into();
            content
        })
        .into()
    }
}
