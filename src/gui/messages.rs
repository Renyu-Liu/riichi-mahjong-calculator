use crate::implements::game::AgariType;
use crate::implements::hand::MentsuType;
use crate::implements::input::OpenMeldInput;
use crate::implements::tiles::{Hai, Kaze};

#[derive(Debug, Clone)]
pub enum Message {
    // Phase 1 Messages
    AddTile(Hai),
    RemoveTile(usize),
    ConfirmHand,
    ModifyHand,
    // Phase 2 Messages
    StartSelectWinningTile,
    SelectWinningTile(Hai),
    SelectMeldType(MentsuType),
    SelectCompleteMeld(OpenMeldInput),
    StartAddClosedKan,
    SelectClosedKan(Hai),
    EditOpenMeld(usize),
    EditClosedKan(usize),
    RemoveOpenMeld(usize),
    RemoveClosedKan(usize),
    // Phase 3 Messages
    ToggleAgariType(AgariType),
    SetBakaze(Kaze),
    SetJikaze(Kaze),
    ToggleRiichi(bool),
    ToggleDoubleRiichi(bool),
    ToggleIppatsu(bool),
    ToggleRinshan(bool),
    ToggleChankan(bool),
    ToggleHaitei(bool),
    ToggleHoutei(bool),
    ToggleTenhou(bool),
    ToggleChiihou(bool),
    ToggleRenhou(bool),
    IncrementHonba,
    DecrementHonba,
    StartAddDora,
    SelectDora(Hai),
    StartAddUraDora,
    SelectUraDora(Hai),
    CalculateScore,
    StartOver,
    // Akadora Messages
    IncrementAkadora,
    DecrementAkadora,
}
