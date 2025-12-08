use super::super::super::components::{cancel_button, create_grid, tile_button};
use super::super::super::messages::Message;
use super::super::super::state::RiichiGui;
use super::super::super::styles::ColoredButtonStyle;
use iced::Element;
use iced::widget::{column, text};

impl RiichiGui {
    pub fn view_selecting_dora(&self, is_ura: bool) -> Element<'_, Message> {
        iced::widget::lazy(is_ura, move |is_ura| {
            let mut tiles = Vec::with_capacity(34);

            for i in 0..34 {
                let tile = crate::implements::tiles::index_to_tile(i);
                let handle = self
                    .tile_images
                    .get(&tile)
                    .expect("Tile image not found")
                    .clone();

                let btn = tile_button(
                    iced::widget::Image::new(handle).width(40).into(),
                    if *is_ura {
                        Message::SelectUraDora(tile)
                    } else {
                        Message::SelectDora(tile)
                    },
                    ColoredButtonStyle::NEUTRAL_HOVER,
                );

                tiles.push(btn);
            }

            let content: Element<'static, Message> = column![
                text("Select").size(24),
                create_grid(tiles, 9),
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
