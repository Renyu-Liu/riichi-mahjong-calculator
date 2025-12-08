use crate::implements::types::hand::AgariHand;
use crate::implements::types::tiles::{Hai, Suhai, TILES_PER_SUIT};
use crate::implements::yaku_checkers::standard::color::check_chinitsu;
use crate::implements::yaku_checkers::utils::get_all_tiles;

pub fn check_chuuren(hand: &AgariHand) -> Option<bool> {
    let all_tiles = get_all_tiles(hand);

    // chinitsu check
    let (is_chinitsu, suit) = check_chinitsu(&all_tiles);
    if !is_chinitsu {
        return None;
    }
    let suit = suit.unwrap();

    // minchou check
    if !hand.mentsu.iter().all(|m| !m.is_minchou) {
        return None;
    }

    // count check
    let mut counts = [0u8; TILES_PER_SUIT];
    for tile in &all_tiles {
        if let Hai::Suhai(Suhai { number: n, suit: s }) = tile {
            if *s == suit {
                counts[(n - 1) as usize] += 1;
            }
        }
    }

    // extra tile check
    let mut has_extra = false;
    let mut extra_tile_num = 0;

    for (i, &count) in counts.iter().enumerate() {
        let num = i + 1;
        let required_count = if num == 1 || num == 9 { 3 } else { 1 };

        if count < required_count {
            return None;
        }
        if count == required_count + 1 {
            if has_extra {
                return None;
            }
            has_extra = true;
            extra_tile_num = num;
        } else if count > required_count + 1 {
            return None;
        }
    }
    if !has_extra {
        return None;
    }

    // junsei check
    if let Hai::Suhai(Suhai { number: n, suit: s }) = hand.agari_hai {
        if s == suit && n as usize == extra_tile_num {
            return Some(true);
        }
    }
    // not junsei
    Some(false)
}
