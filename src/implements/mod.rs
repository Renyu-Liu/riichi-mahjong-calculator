pub mod types;
pub use types::*;
pub mod raw_hand_organizer;
pub use raw_hand_organizer::*;
pub mod yaku_checkers;
pub use yaku_checkers::*;
pub mod score_calculator;
pub use score_calculator::*;

use crate::implements::input::UserInput;
use crate::implements::scoring::AgariResult;

pub fn calculate_agari(input: &UserInput) -> Result<AgariResult, &'static str> {
    // receive input
    let player = &input.player_context;
    let game = &input.game_context;
    let agari_type = input.agari_type;

    // organize hand
    let organizations = organize_hand(input)?;

    // check yaku
    let mut best_result: Option<AgariResult> = None;
    let mut best_payment = 0;

    for organization in organizations {
        if let Ok(yaku_result) = check_all_yaku(organization, player, game, agari_type) {
            let final_score = calculate_score(yaku_result, player, game, agari_type);

            if final_score.total_payment >= best_payment {
                best_payment = final_score.total_payment;
                best_result = Some(final_score);
            }
        }
    }

    // return result
    match best_result {
        Some(res) => Ok(res),
        None => Err("No valid Yaku found"),
    }
}
