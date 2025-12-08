use crate::implements::types::{
    hand::Mentsu,
    tiles::{Hai, Suhai, Suit},
};
use std::collections::{HashMap, HashSet};

pub fn check_ittsu<'a>(shuntsu: &[&'a Mentsu]) -> bool {
    if shuntsu.len() < 3 {
        return false;
    }

    let mut suits: HashMap<Suit, HashSet<u8>> = HashMap::new();

    for m in shuntsu {
        if let Hai::Suhai(Suhai { number: n, suit: s }) = m.tiles[0] {
            suits.entry(s).or_default().insert(n);
        }
    }

    // 123, 456, 789
    for set in suits.values() {
        if set.contains(&1) && set.contains(&4) && set.contains(&7) {
            return true;
        }
    }
    false
}
