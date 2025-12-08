use crate::implements::types::hand::Mentsu;
use std::collections::HashMap;

pub fn check_peikou<'a>(shuntsu: &[&'a Mentsu]) -> (bool, bool) {
    if shuntsu.len() < 2 {
        return (false, false);
    }

    // doublepair check
    let mut counts = HashMap::new();
    for m in shuntsu {
        *counts.entry(m.tiles[0]).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for count in counts.values() {
        pairs += count / 2;
    }

    // (iipeikou, ryanpeikou)
    (pairs == 1, pairs == 2)
}
