// Submodules containing phase-specific view logic
mod composition;
mod definition;
mod result;

use super::super::messages::Message;
use super::super::state::{Phase, RiichiGui};
use super::View;
use crate::gui::styles::ColoredButtonStyle;
use crate::implements::hand::MentsuType;
use iced::widget::{container, scrollable};
use iced::{Color, Element, Length, theme};

impl View for RiichiGui {
    fn view(&self) -> Element<'_, Message> {
        let content = match &self.phase {
            Phase::Composition => composition::build_composition_view(self),
            Phase::Definition => definition::build_definition_view(self),
            Phase::SelectingWinningTile => self.view_selecting_winning_tile(),
            Phase::SelectingMeldTile(m_type, _) => self.view_selecting_meld_tile(*m_type),
            Phase::SelectingClosedKan { .. } => self.view_selecting_closed_kan(),
            Phase::SelectingDora => self.view_selecting_dora(false),
            Phase::SelectingUraDora => self.view_selecting_dora(true),
            Phase::Result => result::build_result_view(self),
        };

        let needs_scroll = !matches!(self.phase, Phase::Composition);

        let main_content = if needs_scroll {
            container(
                scrollable(container(content).width(Length::Fill).center_x()).width(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
        } else {
            container(container(content).width(Length::Fill).center_x())
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .padding(20)
        };

        let help_button = iced::widget::button(iced::widget::text("Rules").size(20))
            .style(theme::Button::Custom(Box::new(ColoredButtonStyle {
                background_color: Color::from_rgb(0.6, 0.6, 0.6),
                text_color: Color::WHITE,
            })))
            .on_press(Message::ShowRules)
            .padding(10);

        let main_view = container(iced::widget::column![
            iced::widget::row![iced::widget::horizontal_space(), help_button].padding(10),
            main_content
        ])
        .width(Length::Fill)
        .height(Length::Fill);

        if self.show_rules {
            self.view_rules_overlay()
        } else {
            main_view.into()
        }
    }

    fn view_composition(&self) -> Element<'_, Message> {
        composition::build_composition_view(self)
    }

    fn view_definition(&self) -> Element<'_, Message> {
        definition::build_definition_view(self)
    }

    fn view_result(&self) -> Element<'_, Message> {
        result::build_result_view(self)
    }

    fn view_selecting_winning_tile(&self) -> Element<'_, Message> {
        self.view_selecting_winning_tile()
    }

    fn view_selecting_meld_tile(&self, m_type: MentsuType) -> Element<'_, Message> {
        self.view_selecting_meld_tile(m_type)
    }

    fn view_selecting_closed_kan(&self) -> Element<'_, Message> {
        self.view_selecting_closed_kan()
    }

    fn view_selecting_dora(&self, is_ura: bool) -> Element<'_, Message> {
        self.view_selecting_dora(is_ura)
    }

    fn view_hand_preview(&self) -> Element<'_, Message> {
        self.view_hand_preview()
    }

    fn view_hand_preview_locked(&self) -> Element<'_, Message> {
        self.view_hand_preview_locked()
    }

    fn view_tile_pool(&self) -> Element<'_, Message> {
        self.view_tile_pool()
    }
}
