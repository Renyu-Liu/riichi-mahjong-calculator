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
                    ColoredButtonStyle::NEUTRAL_HOVER,
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
            MentsuType::Kantsu => unreachable!(""),
        };

        if possible_melds.is_empty() {
            return column![
                match m_type {
                    MentsuType::Koutsu => text("No Triplet available").size(24),
                    MentsuType::Shuntsu => text("No Sequence available").size(24),
                    MentsuType::Kantsu => unreachable!(""),
                },
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
            match m_type {
                MentsuType::Koutsu => text("Select Triplet").size(24),
                MentsuType::Shuntsu => text("Select Sequence").size(24),
                MentsuType::Kantsu => unreachable!(""),
            },
            create_grid(meld_buttons, 5),
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
                ColoredButtonStyle::NEUTRAL_HOVER,
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
    pub fn view_selecting_open_kan(&self) -> Element<'_, Message> {
        let possible_kans = self.get_all_possible_kans();

        if possible_kans.is_empty() {
            return column![text("No Open Kan available").size(24), cancel_button()]
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
                    .enumerate()
                    .map(|(i, t)| {
                        if i == 3 {
                            let handle = self
                                .tile_images_sideways
                                .get(t)
                                .expect("Tile image not found")
                                .clone();
                            // Sideways tile
                            iced::widget::Image::new(handle).height(50).into()
                        } else {
                            let handle = self
                                .tile_images
                                .get(t)
                                .expect("Tile image not found")
                                .clone();
                            iced::widget::Image::new(handle).width(50).into()
                        }
                    })
                    .collect::<Vec<Element<Message>>>())
                .spacing(2)
                .align_items(iced::Alignment::End);

                tile_button(
                    container(tile_images)
                        .padding(5)
                        .style(theme::Container::Box)
                        .into(),
                    Message::SelectOpenKan(*tile),
                    ColoredButtonStyle::NEUTRAL_HOVER,
                )
                .into()
            })
            .collect();

        column![
            text("Select Open Kan").size(24),
            create_grid(kan_buttons, 5),
            cancel_button()
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    pub fn view_selecting_kan_type(&self) -> Element<'_, Message> {
        column![
            text("Select Kan Type").size(24),
            row![
                super::super::components::action_button(
                    "Closed Kan",
                    Message::StartSelectingClosedKan,
                    ColoredButtonStyle::INFO
                ),
                super::super::components::action_button(
                    "Open Kan",
                    Message::StartAddOpenKan,
                    ColoredButtonStyle::INFO
                ),
                super::super::components::action_button(
                    "Added Kan",
                    Message::StartSelectingAddedKan,
                    ColoredButtonStyle::INFO
                ),
            ]
            .spacing(20),
            cancel_button()
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    pub fn view_selecting_closed_kan(&self) -> Element<'_, Message> {
        let possible_kans = self.get_all_possible_kans();

        if possible_kans.is_empty() {
            return column![text("No Closed Kan available").size(24), cancel_button()]
                .spacing(20)
                .align_items(iced::Alignment::Center)
                .into();
        }

        let kan_buttons: Vec<Element<Message>> = possible_kans
            .iter()
            .map(|tile| {
                // [Back, Tile, Tile, Back]
                let mut tile_elements: Vec<Element<Message>> = Vec::new();

                // Back
                if let Some(handle) = &self.tile_back_image {
                    tile_elements.push(iced::widget::Image::new(handle.clone()).width(50).into());
                }

                // Tile x2
                for _ in 0..2 {
                    let handle = self
                        .tile_images
                        .get(tile)
                        .expect("Tile image not found")
                        .clone();
                    tile_elements.push(iced::widget::Image::new(handle).width(50).into());
                }

                // Back
                if let Some(handle) = &self.tile_back_image {
                    tile_elements.push(iced::widget::Image::new(handle.clone()).width(50).into());
                }

                tile_button(
                    container(row(tile_elements).spacing(2))
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
            text("Select Closed Kan").size(24),
            create_grid(kan_buttons, 5),
            cancel_button()
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    pub fn view_selecting_added_kan(&self) -> Element<'_, Message> {
        // check pons
        let has_pon = self
            .open_melds
            .iter()
            .any(|m| m.mentsu_type == MentsuType::Koutsu);

        if !has_pon {
            return column![
                text("Select a Pon before you select an added Kan.").size(24),
                cancel_button()
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into();
        }

        // check addable pons
        let pons: Vec<(usize, &crate::implements::input::OpenMeldInput)> = self
            .open_melds
            .iter()
            .enumerate()
            .filter(|(_, m)| {
                m.mentsu_type == MentsuType::Koutsu
                    && self
                        .hand_tiles
                        .iter()
                        .filter(|&&t| t == m.representative_tile)
                        .count()
                        >= 4
            })
            .collect();

        if pons.is_empty() {
            return column![
                text("No available tiles to form an added Kan.").size(24),
                cancel_button()
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into();
        }

        let kan_buttons: Vec<Element<Message>> = pons
            .iter()
            .map(|(idx, m)| {
                let mut tiles = self.get_meld_tiles(m);
                if tiles.len() > 0 {
                    tiles.push(tiles[0]);
                }

                let tile_images = if tiles.len() == 4 {
                    let first = tiles[0];
                    let stack_bottom = tiles[1];
                    let stack_top = tiles[3];
                    let last = tiles[2];

                    let img_first = iced::widget::Image::new(
                        self.tile_images
                            .get(&first)
                            .expect("Tile image not found")
                            .clone(),
                    )
                    .width(50);

                    // Stacked sideways tiles
                    let img_stack_bottom = iced::widget::Image::new(
                        self.tile_images_sideways
                            .get(&stack_bottom)
                            .expect("Tile image not found")
                            .clone(),
                    )
                    .height(50);

                    let img_stack_top = iced::widget::Image::new(
                        self.tile_images_sideways
                            .get(&stack_top)
                            .expect("Tile image not found")
                            .clone(),
                    )
                    .height(50);

                    let img_last = iced::widget::Image::new(
                        self.tile_images
                            .get(&last)
                            .expect("Tile image not found")
                            .clone(),
                    )
                    .width(50);

                    // 1 Up, 2 Stack(Top, Bottom), 1 Down
                    row![
                        img_first,
                        column![img_stack_top, img_stack_bottom]
                            .align_items(iced::Alignment::Center),
                        img_last
                    ]
                    .align_items(iced::Alignment::End)
                    .spacing(2)
                } else {
                    row(tiles
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
                    .spacing(2)
                };

                tile_button(
                    container(tile_images)
                        .padding(5)
                        .style(theme::Container::Box)
                        .into(),
                    Message::SelectAddedKan(*idx),
                    ColoredButtonStyle::NEUTRAL_HOVER,
                )
                .into()
            })
            .collect();

        column![
            text("Select Added Kan").size(24),
            create_grid(kan_buttons, 5),
            cancel_button()
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }
}
