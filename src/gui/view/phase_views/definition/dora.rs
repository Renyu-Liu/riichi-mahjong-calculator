use super::common::section_header;
use crate::gui::components::{action_button, tile_image_button};
use crate::gui::messages::Message;
use crate::gui::state::RiichiGui;
use crate::gui::styles::ColoredButtonStyle;
use iced::widget::{button, column, row, text};
use iced::{Color, Element, theme};

pub fn build_dora_section(gui: &RiichiGui) -> Element<'_, Message> {
    column![
        section_header("Dora"),
        column![
            text("Dora:"),
            row(gui
                .dora_indicators
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    let handle = gui
                        .tile_images
                        .get(t)
                        .expect("Tile image not found")
                        .clone();
                    tile_image_button(
                        handle,
                        30,
                        Message::RemoveDora(i),
                        theme::Button::Custom(Box::new(ColoredButtonStyle::NEUTRAL_HOVER)),
                    )
                })
                .collect::<Vec<Element<Message>>>())
            .spacing(5),
            action_button("Add", Message::StartAddDora, ColoredButtonStyle::INFO,),
            if gui.is_riichi {
                column![
                    text("Ura Dora:"),
                    row(gui
                        .uradora_indicators
                        .iter()
                        .enumerate()
                        .map(|(i, t)| {
                            let handle = gui
                                .tile_images
                                .get(t)
                                .expect("Tile image not found")
                                .clone();
                            tile_image_button(
                                handle,
                                30,
                                Message::RemoveUraDora(i),
                                theme::Button::Custom(Box::new(ColoredButtonStyle::NEUTRAL_HOVER)),
                            )
                        })
                        .collect::<Vec<Element<Message>>>())
                    .spacing(5),
                    action_button("Add", Message::StartAddUraDora, ColoredButtonStyle::INFO,),
                ]
                .spacing(5)
                .align_items(iced::Alignment::Center)
            } else {
                column![]
            }
        ]
        .spacing(5)
        .align_items(iced::Alignment::Center),
        {
            let max_akadora = gui.get_max_akadora_count();

            if max_akadora > 0 {
                row![
                    text(format!("Red Dora: {}", gui.num_akadora)),
                    button(text("+"))
                        .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                            background_color: Color::from_rgb(0.0, 0.0, 0.6),
                            text_color: Color::WHITE,
                            hover_color: None,
                        })))
                        .on_press_maybe(if gui.num_akadora < max_akadora && gui.num_akadora < 4 {
                            Some(Message::IncrementAkadora)
                        } else {
                            None
                        }),
                    button(text("-"))
                        .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                            background_color: Color::from_rgb(0.6, 0.0, 0.0),
                            text_color: Color::WHITE,
                            hover_color: None,
                        })))
                        .on_press_maybe(if gui.num_akadora > 0 {
                            Some(Message::DecrementAkadora)
                        } else {
                            None
                        }),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center)
            } else {
                row![]
            }
        },
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .into()
}
