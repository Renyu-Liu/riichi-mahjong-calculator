use super::super::components::create_grid;
use super::super::messages::Message;
use super::super::state::RiichiGui;
use super::super::styles::ColoredButtonStyle;
use crate::implements::hand::MentsuType;
use crate::implements::tiles::Hai;
use iced::widget::{button, column, container, row, text};
use iced::{Color, Element, theme};

impl RiichiGui {
    /// Renders selecting winning tile view
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
                button(iced::widget::Image::new(handle).width(50))
                    .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                        background_color: Color::WHITE,
                        text_color: Color::BLACK,
                    })))
                    .on_press(Message::SelectWinningTile(*tile))
                    .padding(5)
                    .into()
            })
            .collect();

        column![
            text("Select Winning Tile").size(24),
            create_grid(tiles, 10),
            button(text("Cancel"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::CancelSelection)
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    /// Renders selecting meld view
    pub fn view_selecting_meld_tile(&self, m_type: MentsuType) -> Element<'_, Message> {
        use super::super::state::Phase;

        let editing_idx = if let Phase::SelectingMeldTile(_, idx) = &self.phase {
            *idx
        } else {
            None
        };

        let possible_melds = match m_type {
            MentsuType::Koutsu => self.get_all_possible_pons(editing_idx),
            MentsuType::Shuntsu => self.get_all_possible_chiis(editing_idx),
            MentsuType::Kantsu => {
                vec![]
            }
        };

        if possible_melds.is_empty() {
            return column![
                text(format!("No valid {:?} available", m_type)).size(24),
                button(text("Cancel"))
                    .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                        background_color: Color::from_rgb(0.6, 0.0, 0.0),
                        text_color: Color::WHITE,
                    })))
                    .on_press(Message::CancelSelection)
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

                let meld_button = button(
                    container(tile_images)
                        .padding(5)
                        .style(theme::Container::Box),
                )
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.95, 0.95, 0.95),
                    text_color: Color::BLACK,
                })))
                .on_press(Message::SelectCompleteMeld(meld.clone()))
                .padding(3);

                meld_button.into()
            })
            .collect();

        column![
            text(format!("Select {:?}", m_type)).size(24),
            create_grid(meld_buttons, 5),
            button(text("Cancel"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::CancelSelection)
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    /// Renders selecting closed Kan view
    pub fn view_selecting_closed_kan(&self) -> Element<'_, Message> {
        let possible_kans = self.get_all_possible_kans();

        if possible_kans.is_empty() {
            return column![
                text("No valid Kantsu available").size(24),
                button(text("Cancel"))
                    .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                        background_color: Color::from_rgb(0.6, 0.0, 0.0),
                        text_color: Color::WHITE,
                    })))
                    .on_press(Message::CancelSelection)
            ]
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

                let kan_button = button(
                    container(tile_images)
                        .padding(5)
                        .style(theme::Container::Box),
                )
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.95, 0.95, 0.95),
                    text_color: Color::BLACK,
                })))
                .on_press(Message::SelectClosedKan(*tile))
                .padding(3);

                kan_button.into()
            })
            .collect();

        column![
            text("Select Kantsu").size(24),
            create_grid(kan_buttons, 5),
            button(text("Cancel"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::CancelSelection)
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    /// Renders selecting Dora view
    pub fn view_selecting_dora(&self, is_ura: bool) -> Element<'_, Message> {
        let mut tiles = Vec::new();

        for i in 0..34 {
            let tile = crate::implements::tiles::index_to_tile(i);
            let handle = self
                .tile_images
                .get(&tile)
                .expect("Tile image not found")
                .clone();

            let btn = button(iced::widget::Image::new(handle).width(40))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::WHITE,
                    text_color: Color::BLACK,
                })))
                .on_press(if is_ura {
                    Message::SelectUraDora(tile)
                } else {
                    Message::SelectDora(tile)
                })
                .padding(5)
                .into();

            tiles.push(btn);
        }

        column![
            text("Select").size(24),
            create_grid(tiles, 9),
            button(text("Cancel"))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: Color::from_rgb(0.6, 0.0, 0.0),
                    text_color: Color::WHITE,
                })))
                .on_press(Message::CancelSelection)
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }
}
