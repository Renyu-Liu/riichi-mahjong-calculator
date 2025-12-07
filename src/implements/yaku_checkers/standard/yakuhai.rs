use crate::implements::{
    types::{
        game::{GameContext, PlayerContext},
        hand::AgariHand,
        tiles::{Hai, Jihai, Sangenpai},
        yaku::Yaku,
    },
    yaku_checkers::utils::is_koutsu_or_kantsu,
};
use std::collections::HashSet;

pub fn check_yakuhai(hand: &AgariHand, player: &PlayerContext, game: &GameContext) -> Vec<Yaku> {
    let mut yaku = Vec::new();

    let koutsu_tiles: HashSet<Hai> = hand
        .mentsu
        .iter()
        .filter(|m| is_koutsu_or_kantsu(m))
        .map(|m| m.tiles[0])
        .collect();

    // Dragons
    if koutsu_tiles.contains(&Hai::Jihai(Jihai::Sangen(Sangenpai::Haku))) {
        yaku.push(Yaku::YakuhaiHaku);
    }
    if koutsu_tiles.contains(&Hai::Jihai(Jihai::Sangen(Sangenpai::Hatsu))) {
        yaku.push(Yaku::YakuhaiHatsu);
    }
    if koutsu_tiles.contains(&Hai::Jihai(Jihai::Sangen(Sangenpai::Chun))) {
        yaku.push(Yaku::YakuhaiChun);
    }

    // Winds
    let bakaze_hai = Hai::Jihai(Jihai::Kaze(game.bakaze));
    if koutsu_tiles.contains(&bakaze_hai) {
        yaku.push(Yaku::YakuhaiBakaze);
    }

    let jikaze_hai = Hai::Jihai(Jihai::Kaze(player.jikaze));
    if koutsu_tiles.contains(&jikaze_hai) && jikaze_hai != bakaze_hai {
        yaku.push(Yaku::YakuhaiJikaze);
    } else if koutsu_tiles.contains(&jikaze_hai) && jikaze_hai == bakaze_hai {
        yaku.push(Yaku::YakuhaiJikaze);
    }

    yaku
}
