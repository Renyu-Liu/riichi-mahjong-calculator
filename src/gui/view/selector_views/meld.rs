use super::super::super::components::{cancel_button, create_grid, tile_button};
use super::super::super::messages::Message;
use super::super::super::state::RiichiGui;
use super::super::super::styles::ColoredButtonStyle;
use crate::implements::hand::MentsuType;
use iced::widget::{column, container, row, text};
use iced::{Element, theme};

impl RiichiGui {
    pub fn view_selecting_meld_tile(&self, m_type: MentsuType) -> Element<'_, Message> {
        iced::widget::lazy(
            (m_type, self.hand_tiles.clone()),
            move |(m_type, _hand_tiles)| {
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

                let content: Element<'static, Message> = column![
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
                .into();
                content
            },
        )
        .into()
    }
}
