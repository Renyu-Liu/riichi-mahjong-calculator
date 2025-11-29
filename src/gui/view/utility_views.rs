use super::super::messages::Message;

use super::super::state::RiichiGui;
use super::super::styles::ColoredButtonStyle;
use iced::widget::{button, column, container, image, row, text};
use iced::{Color, Element, Length, theme};

impl RiichiGui {
    /// Renders hand preview
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
                button(iced::widget::Image::new(handle).width(40))
                    .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                        background_color: Color::WHITE,
                        text_color: Color::BLACK,
                    })))
                    .on_press(Message::RemoveTile(i))
                    .padding(0)
                    .into()
            })
            .collect();

        container(row(tiles).spacing(5))
            .height(Length::Fixed(80.0))
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }

    /// Renders locked hand preview
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

                let btn = button(iced::widget::Image::new(handle).width(40))
                    .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                        background_color: Color::WHITE,
                        text_color: Color::BLACK,
                    })))
                    .padding(0);

                btn.into()
            })
            .collect();

        row(tiles).spacing(5).into()
    }

    /// Renders tile pool
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

    /// Renders rules overlay
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

        let close_button = button(text("Close").size(20))
            .on_press(Message::HideRules)
            .padding(10)
            .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                background_color: Color::from_rgb(0.6, 0.0, 0.0),
                text_color: Color::WHITE,
            })));

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
