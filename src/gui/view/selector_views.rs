use super::super::components::{cancel_button, create_grid, tile_button};
use super::super::messages::Message;
use super::super::state::RiichiGui;
use super::super::styles::ColoredButtonStyle;
use crate::implements::hand::MentsuType;
use crate::implements::tiles::Hai;
use iced::widget::{column, container, row, text};
use iced::{Element, theme};

impl RiichiGui {
    pub fn view_selecting_winning_tile(&self) -> Element<'_, Message> {
        let mut unique_tiles: Vec<Hai> = self.hand_tiles.iter().map(|t| *t).collect();
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
                    ColoredButtonStyle::NEUTRAL,
                )
            })
            .collect();

        column![
            text("Select Winning Tile").size(24),
            create_grid(tiles, 10),
            cancel_button()
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    pub fn view_selecting_meld_tile(&self, m_type: MentsuType) -> Element<'_, Message> {
        let possible_melds = match m_type {
            MentsuType::Koutsu => self.get_all_possible_pons(),
            MentsuType::Shuntsu => self.get_all_possible_chiis(),
            MentsuType::Kantsu => {
                vec![]
            }
        };

        if possible_melds.is_empty() {
            return column![
                text(format!("No valid {:?} available", m_type)).size(24),
                cancel_button()
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into();
        }

        let meld_buttons: Vec<Element<Message>> = possible_melds
            .iter()
            .map(|meld| {
                let tiles = self.get_meld_tiles(meld);
                let tile_images = row(tiles
                    .iter()
                    .map(|t| {
                        let handle = self
                            .tile_images
                            .get(t)
                            .expect("Tile image not found")
                            .clone();
                        iced::widget::Image::new(handle).width(50).into()
                    })
                    .collect::<Vec<Element<Message>>>())
                .spacing(2);

                tile_button(
                    container(tile_images)
                        .padding(5)
                        .style(theme::Container::Box)
                        .into(),
                    Message::SelectCompleteMeld(meld.clone()),
                    ColoredButtonStyle::NEUTRAL_HOVER,
                )
                .into()
            })
            .collect();

        column![
            text(format!("Select {:?}", m_type)).size(24),
            create_grid(meld_buttons, 5),
            cancel_button()
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    pub fn view_selecting_closed_kan(&self) -> Element<'_, Message> {
        let possible_kans = self.get_all_possible_kans();

        if possible_kans.is_empty() {
            return column![text("No valid Kantsu available").size(24), cancel_button()]
                .spacing(20)
                .align_items(iced::Alignment::Center)
                .into();
        }

        let kan_buttons: Vec<Element<Message>> = possible_kans
            .iter()
            .map(|tile| {
                let tiles = vec![*tile; 4];
                let tile_images = row(tiles
                    .iter()
                    .map(|t| {
                        let handle = self
                            .tile_images
                            .get(t)
                            .expect("Tile image not found")
                            .clone();
                        iced::widget::Image::new(handle).width(50).into()
                    })
                    .collect::<Vec<Element<Message>>>())
                .spacing(2);

                tile_button(
                    container(tile_images)
                        .padding(5)
                        .style(theme::Container::Box)
                        .into(),
                    Message::SelectClosedKan(*tile),
                    ColoredButtonStyle::NEUTRAL_HOVER,
                )
                .into()
            })
            .collect();

        column![
            text("Select Kantsu").size(24),
            create_grid(kan_buttons, 5),
            cancel_button()
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    pub fn view_selecting_dora(&self, is_ura: bool) -> Element<'_, Message> {
        let mut tiles = Vec::new();

        for i in 0..34 {
            let tile = crate::implements::tiles::index_to_tile(i);
            let handle = self
                .tile_images
                .get(&tile)
                .expect("Tile image not found")
                .clone();

            let btn = tile_button(
                iced::widget::Image::new(handle).width(40).into(),
                if is_ura {
                    Message::SelectUraDora(tile)
                } else {
                    Message::SelectDora(tile)
                },
                ColoredButtonStyle::NEUTRAL,
            );

            tiles.push(btn);
        }

        column![
            text("Select").size(24),
            create_grid(tiles, 9),
            cancel_button()
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }
}
