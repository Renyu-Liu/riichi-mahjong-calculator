use crate::implements::{types::hand::AgariHand, yaku_checkers::utils::get_all_tiles};

pub fn check_tanyao(hand: &AgariHand) -> bool {
    get_all_tiles(hand).iter().all(|t| t.is_simple())
}
