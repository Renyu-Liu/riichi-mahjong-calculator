pub mod common;
pub mod dora;
pub mod game_info;
pub mod melds;
pub mod special_yaku;
pub mod winning_tile;

use self::{
    dora::build_dora_section, game_info::build_game_info_section, melds::build_melds_section,
    special_yaku::build_special_yaku_section, winning_tile::build_winning_tile_section,
};
use crate::gui::components::action_button;
use crate::gui::messages::Message;
use crate::gui::state::RiichiGui;
use crate::gui::styles::ColoredButtonStyle;
use iced::widget::{button, column, text};
use iced::{Color, Element, theme};

pub fn build_definition_view(gui: &RiichiGui) -> Element<'_, Message> {
    let hand_preview = gui.view_hand_preview_locked();
    let modify_btn = action_button("Modify Hand", Message::ModifyHand, ColoredButtonStyle::INFO);

    let calculate_btn = button(text("Calculate Score"))
        .style(theme::Button::Custom(Box::new(ColoredButtonStyle::PRIMARY)))
        .on_press_maybe(if gui.winning_tile.is_some() {
            Some(Message::CalculateScore)
        } else {
            None
        });

    let mut content = column![
        hand_preview,
        modify_btn,
        iced::widget::rule::Rule::horizontal(30),
        build_winning_tile_section(gui),
        iced::widget::rule::Rule::horizontal(30),
        build_melds_section(gui),
        iced::widget::rule::Rule::horizontal(30),
        build_game_info_section(gui),
        iced::widget::rule::Rule::horizontal(30),
        build_special_yaku_section(gui),
        iced::widget::rule::Rule::horizontal(30),
        build_dora_section(gui),
        calculate_btn
    ];

    if gui.winning_tile.is_none() {
        content = content.push(
            text("You must select a winning tile")
                .size(12)
                .style(Color::from_rgb(0.8, 0.0, 0.0)),
        );
    }

    content
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
}
