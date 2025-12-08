use super::super::messages::Message;

use super::super::components::{action_button, tile_image, tile_image_button};
use super::super::state::RiichiGui;
use super::super::styles::ColoredButtonStyle;
use iced::widget::{button, column, container, image, row, text};
use iced::{Color, Element, Length, theme};

impl RiichiGui {
    /// in composition phase
    pub fn view_hand_preview(&self) -> Element<'_, Message> {
        let tiles: Vec<Element<Message>> = self
            .hand_tiles
            .iter()
            .enumerate()
            .map(|(i, tile)| {
                let handle = self
                    .tile_images
                    .get(tile)
                    .expect("Tile image not found")
                    .clone();
                tile_image_button(
                    handle,
                    40,
                    Message::RemoveTile(i),
                    theme::Button::Custom(Box::new(ColoredButtonStyle::NEUTRAL_HOVER)),
                )
            })
            .collect();

        container(row(tiles).spacing(5))
            .height(Length::Fixed(80.0))
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }

    /// in definition & result phase
    pub fn view_hand_preview_locked(&self) -> Element<'_, Message> {
        let tiles: Vec<Element<Message>> = self
            .hand_tiles
            .iter()
            .enumerate()
            .map(|(_, tile)| {
                let handle = self
                    .tile_images
                    .get(tile)
                    .expect("Tile image not found")
                    .clone();
                tile_image(handle, 40)
            })
            .collect();

        row(tiles).spacing(5).into()
    }

    /// in composition phase
    pub fn view_tile_pool(&self) -> Element<'_, Message> {
        let mut tiles = Vec::new();

        for i in 0..34 {
            let tile = crate::implements::tiles::index_to_tile(i);
            let count = self.tile_counts[i];
            let handle = self
                .tile_images
                .get(&tile)
                .expect("Tile image not found")
                .clone();

            let tile_image = image(handle).width(50);

            let count_text = text(format!("({})", count)).size(12).style(if count > 0 {
                Color::BLACK
            } else {
                Color::from_rgb(0.5, 0.5, 0.5)
            });

            let button_bg_color = if count > 0 {
                Color::WHITE
            } else {
                Color::from_rgb(0.85, 0.85, 0.85)
            };

            let btn = button(column![tile_image, count_text].align_items(iced::Alignment::Center))
                .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                    background_color: button_bg_color,
                    text_color: Color::BLACK,
                    hover_color: if count > 0 {
                        Some(Color::from_rgb(0.8, 0.8, 0.8))
                    } else {
                        None
                    },
                })))
                .on_press_maybe(if count > 0 {
                    Some(Message::AddTile(tile))
                } else {
                    None
                })
                .padding(5)
                .into();

            tiles.push(btn);
        }

        super::super::components::create_grid(tiles, 9)
    }

    /// every phase
    pub fn view_rules_overlay(&self) -> Element<'_, Message> {
        let rules_image = if let Some(handle) = &self.rules_image {
            iced::widget::scrollable(
                iced::widget::image(handle.clone())
                    .width(Length::Fill)
                    .content_fit(iced::ContentFit::Contain),
            )
            .height(Length::Fill)
            .width(Length::Fill)
        } else {
            iced::widget::scrollable(
                iced::widget::image("assets/riichi_rule.png")
                    .width(Length::Fill)
                    .content_fit(iced::ContentFit::Contain),
            )
            .height(Length::Fill)
            .width(Length::Fill)
        };

        let close_button = action_button("Close", Message::HideRules, ColoredButtonStyle::DANGER);

        container(
            column![
                row![iced::widget::horizontal_space(), close_button]
                    .align_items(iced::Alignment::Center)
                    .padding(10),
                rules_image
            ]
            .align_items(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(
            super::super::styles::OverlayStyle,
        )))
        .into()
    }
}
