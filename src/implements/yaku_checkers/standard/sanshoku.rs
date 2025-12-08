use crate::implements::{
    types::{
        hand::{AgariHand, Mentsu},
        tiles::{Hai, Suhai, Suit},
    },
    yaku_checkers::utils::is_koutsu_or_kantsu,
};
use std::collections::HashMap;

pub fn check_sanshoku_generic(mentsu_list: &[&Mentsu]) -> bool {
    if mentsu_list.len() < 3 {
        return false;
    }

    // man, pin, sou check
    let mut starters: HashMap<u8, (bool, bool, bool)> = HashMap::new();
    for m in mentsu_list {
        if let Hai::Suhai(Suhai { number: n, suit: s }) = m.tiles[0] {
            let entry = starters.entry(n).or_insert((false, false, false));
            match s {
                Suit::Manzu => entry.0 = true,
                Suit::Pinzu => entry.1 = true,
                Suit::Souzu => entry.2 = true,
            }
        }
    }

    starters.values().any(|&(m, p, s)| m && p && s)
}

pub fn check_sanshoku_doujun<'a>(shuntsu: &[&'a Mentsu]) -> bool {
    check_sanshoku_generic(shuntsu)
}

pub fn check_sanshoku_doukou(hand: &AgariHand) -> bool {
    let koutsu: Vec<&Mentsu> = hand
        .mentsu
        .iter()
        .filter(|m| is_koutsu_or_kantsu(m))
        .collect();

    check_sanshoku_generic(&koutsu)
}
