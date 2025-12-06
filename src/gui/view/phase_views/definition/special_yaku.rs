use super::common::{checkbox_with_conflict, section_header};
use crate::gui::messages::Message;
use crate::gui::state::RiichiGui;
use crate::implements::game::AgariType;
use crate::implements::tiles::Kaze;
use iced::Element;
use iced::widget::{column, row};

pub fn build_special_yaku_section(gui: &RiichiGui) -> Element<'_, Message> {
    let is_oya = gui.jikaze == Kaze::Ton;
    let is_ron = gui.agari_type == AgariType::Ron;
    let is_tsumo = gui.agari_type == AgariType::Tsumo;
    let is_menzen = gui.open_melds.is_empty();

    column![
        section_header("Special Yaku"),
        row![
            checkbox_with_conflict("Riichi", gui.is_riichi, Message::ToggleRiichi, is_menzen),
            checkbox_with_conflict(
                "Double Riichi",
                gui.is_daburu_riichi,
                Message::ToggleDoubleRiichi,
                is_menzen
            ),
            checkbox_with_conflict(
                "Ippatsu",
                gui.is_ippatsu,
                Message::ToggleIppatsu,
                gui.is_riichi || gui.is_daburu_riichi
            ),
        ]
        .spacing(10),
        row![
            checkbox_with_conflict(
                "Tenhou",
                gui.is_tenhou,
                Message::ToggleTenhou,
                is_tsumo && is_oya && is_menzen
            ),
            checkbox_with_conflict(
                "Chiihou",
                gui.is_chiihou,
                Message::ToggleChiihou,
                is_tsumo && !is_oya && is_menzen
            ),
            checkbox_with_conflict(
                "Renhou",
                gui.is_renhou,
                Message::ToggleRenhou,
                is_ron && is_menzen
            ),
        ]
        .spacing(10),
        row![
            checkbox_with_conflict("Haitei", gui.is_haitei, Message::ToggleHaitei, is_tsumo),
            checkbox_with_conflict("Houtei", gui.is_houtei, Message::ToggleHoutei, is_ron),
            checkbox_with_conflict("Rinshan", gui.is_rinshan, Message::ToggleRinshan, is_tsumo),
            checkbox_with_conflict("Chankan", gui.is_chankan, Message::ToggleChankan, is_ron),
        ]
        .spacing(10),
    ]
    .spacing(5)
    .align_items(iced::Alignment::Center)
    .into()
}
