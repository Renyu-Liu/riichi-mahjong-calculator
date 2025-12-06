use crate::implements::{
    types::{
        hand::AgariHand,
        tiles::{Hai, Jihai},
    },
    yaku_checkers::utils::is_koutsu_or_kantsu,
};

pub fn check_shousangen(hand: &AgariHand) -> bool {
    let mut dragon_koutsu = 0;
    let mut dragon_atama = false;

    for m in &hand.mentsu {
        if is_koutsu_or_kantsu(m) {
            if let Hai::Jihai(Jihai::Sangen(_)) = m.tiles[0] {
                dragon_koutsu += 1;
            }
        }
    }

    if let Hai::Jihai(Jihai::Sangen(_)) = hand.atama.0 {
        dragon_atama = true;
    }

    dragon_koutsu == 2 && dragon_atama
}

pub fn check_chanta_junchan(groups: &[Vec<Hai>]) -> (bool, bool) {
    let mut is_chanta = true;
    let mut is_junchan = true;

    for group in groups {
        let mut has_terminal = false;
        let mut has_jihai = false;

        for tile in group {
            if tile.is_jihai() {
                has_jihai = true;
            }
            if tile.is_terminal() {
                has_terminal = true;
            }
        }

        if !has_terminal && !has_jihai {
            is_chanta = false;
            is_junchan = false;
            break;
        }

        if has_jihai {
            is_junchan = false;
        }
    }

    (is_chanta, is_junchan && is_chanta)
}
