use crate::implements::types::tiles::{Hai, Suhai, Suit};

fn check_color(all_tiles: &[Hai]) -> (bool, bool, Option<Suit>) {
    let mut suit = None;
    let mut has_jihai = false;
    let mut is_honitsu = true;
    let mut is_chinitsu = true;

    // color check
    for tile in all_tiles {
        match tile {
            Hai::Suhai(Suhai { suit: s, .. }) => {
                if suit.is_none() {
                    suit = Some(*s);
                } else if suit != Some(*s) {
                    is_honitsu = false;
                    is_chinitsu = false;
                    break;
                }
            }
            Hai::Jihai(_) => {
                has_jihai = true;
                is_chinitsu = false;
            }
        }
    }

    // honor check
    if !has_jihai && suit.is_none() {
        is_honitsu = false;
        is_chinitsu = false;
    }

    if !has_jihai {
        is_honitsu = false;
    }

    (is_honitsu, is_chinitsu, suit)
}

pub fn check_honitsu(all_tiles: &[Hai]) -> (bool, Option<Suit>) {
    let (hon, _chin, suit) = check_color(all_tiles);
    (hon, suit)
}

pub fn check_chinitsu(all_tiles: &[Hai]) -> (bool, Option<Suit>) {
    let (_hon, chin, suit) = check_color(all_tiles);
    (chin, suit)
}
