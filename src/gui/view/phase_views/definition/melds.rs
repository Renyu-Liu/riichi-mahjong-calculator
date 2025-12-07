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
        section_header("Open Hand"),
        column(
            gui.open_melds
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let tiles = gui.get_meld_tiles(m);
                    let tile_images = if m.is_added_kan && tiles.len() == 4 {
                        let first = tiles[0];
                        let stack_bottom = tiles[1];
                        let stack_top = tiles[3];
                        let last = tiles[2];
                        // Added Kan
                        let img_first = iced::widget::Image::new(
                            gui.tile_images
                                .get(&first)
                                .expect("Tile image not found")
                                .clone(),
                        )
                        .width(40);

                        let img_stack_bottom = iced::widget::Image::new(
                            gui.tile_images_sideways
                                .get(&stack_bottom)
                                .expect("Tile image not found")
                                .clone(),
                        )
                        .height(40);

                        let img_stack_top = iced::widget::Image::new(
                            gui.tile_images_sideways
                                .get(&stack_top)
                                .expect("Tile image not found")
                                .clone(),
                        )
                        .height(40);

                        let img_last = iced::widget::Image::new(
                            gui.tile_images
                                .get(&last)
                                .expect("Tile image not found")
                                .clone(),
                        )
                        .width(40);

                        row![
                            img_first,
                            column![img_stack_top, img_stack_bottom]
                                .align_items(iced::Alignment::Center),
                            img_last
                        ]
                        .align_items(iced::Alignment::End)
                        .spacing(2)
                    } else if m.mentsu_type == MentsuType::Kantsu && tiles.len() == 4 {
                        // Open Kan
                        let up_tiles = &tiles[0..3];
                        let sideways_tile = tiles[3];

                        let mut row_imgs = row(up_tiles
                            .iter()
                            .map(|t| {
                                let handle = gui
                                    .tile_images
                                    .get(t)
                                    .expect("Tile image not found")
                                    .clone();
                                iced::widget::Image::new(handle).width(40).into()
                            })
                            .collect::<Vec<Element<Message>>>());

                        let sideways_img = iced::widget::Image::new(
                            gui.tile_images_sideways
                                .get(&sideways_tile)
                                .expect("Tile image not found")
                                .clone(),
                        )
                        .height(40);

                        row_imgs = row_imgs
                            .push(sideways_img)
                            .spacing(2)
                            .align_items(iced::Alignment::End);
                        row_imgs
                    } else {
                        // Pon/Chii
                        row(tiles
                            .iter()
                            .map(|t| {
                                let handle = gui
                                    .tile_images
                                    .get(t)
                                    .expect("Tile image not found")
                                    .clone();
                                crate::gui::components::tile_image(handle, 40)
                            })
                            .collect::<Vec<Element<Message>>>())
                        .spacing(2)
                    };

                    row![
                        button(tile_images)
                            .on_press(Message::RemoveOpenMeld(i))
                            .style(theme::Button::Custom(Box::new(
                                ColoredButtonStyle::NEUTRAL_HOVER
                            )))
                    ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .into()
                })
                .collect::<Vec<Element<Message>>>()
        )
        .align_items(iced::Alignment::Center)
        .spacing(10),
        column(
            gui.closed_kans
                .iter()
                .enumerate()
                .map(|(i, k)| {
                    let mut tile_elements: Vec<Element<Message>> = Vec::new();

                    // Closed Kan
                    if let Some(handle) = &gui.tile_back_image {
                        tile_elements.push(tile_image(handle.clone(), 40));
                    }

                    let handle = gui
                        .tile_images
                        .get(k)
                        .expect("Tile image not found")
                        .clone();
                    for _ in 0..2 {
                        tile_elements.push(tile_image(handle.clone(), 40));
                    }

                    if let Some(handle) = &gui.tile_back_image {
                        tile_elements.push(tile_image(handle.clone(), 40));
                    }

                    let tile_images = row(tile_elements).spacing(2);
                    row![
                        button(tile_images)
                            .on_press(Message::RemoveClosedKan(i))
                            .style(theme::Button::Custom(Box::new(
                                ColoredButtonStyle::NEUTRAL_HOVER
                            )))
                    ]
                    .align_items(iced::Alignment::Center)
                    .into()
                })
                .collect::<Vec<Element<Message>>>()
        )
        .spacing(10),
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
            action_button("Add Kan", Message::StartAddKan, ColoredButtonStyle::INFO,)
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center)
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .into()
}
