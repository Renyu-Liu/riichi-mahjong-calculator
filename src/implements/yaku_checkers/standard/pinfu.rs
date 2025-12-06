use crate::implements::types::{
    game::{GameContext, PlayerContext},
    hand::{AgariHand, Machi, MentsuType},
    tiles::{Hai, Jihai},
};

pub fn check_pinfu(hand: &AgariHand, player: &PlayerContext, game: &GameContext) -> bool {
    // Kantsu check
    if hand
        .mentsu
        .iter()
        .any(|m| m.mentsu_type == MentsuType::Kantsu)
    {
        return false;
    }

    // menzen check
    if !player.is_menzen {
        return false;
    }
    // Shuntsu check
    if !hand
        .mentsu
        .iter()
        .all(|m| m.mentsu_type == MentsuType::Shuntsu)
    {
        return false;
    }
    // Yakuhai check
    if let Hai::Jihai(Jihai::Sangen(_)) = hand.atama.0 {
        return false;
    }
    if let Hai::Jihai(Jihai::Kaze(k)) = hand.atama.0 {
        if k == game.bakaze || k == player.jikaze {
            return false;
        }
    }
    // Ryanmen check
    if hand.machi != Machi::Ryanmen {
        return false;
    }

    true
}
