use super::RiichiGui;
use crate::implements::calculate_agari;
use crate::implements::types::game::{AgariType, GameContext, PlayerContext};
use crate::implements::types::input::UserInput;
use crate::implements::types::tiles::{Hai, Kaze, Suhai, index_to_tile, tile_to_index};

impl RiichiGui {
    pub fn calculate_score_result(&mut self) {
        if let Some(winning_tile) = self.winning_tile {
            let mut counts = self.get_active_hand_counts();

            if self.agari_type == AgariType::Ron {
                let idx = tile_to_index(&winning_tile);
                if counts[idx] > 0 {
                    counts[idx] -= 1;
                }
            }

            let mut hand_tiles = Vec::with_capacity(14);
            for i in 0..34 {
                let count = counts[i];
                if count > 0 {
                    let tile = index_to_tile(i);
                    for _ in 0..count {
                        hand_tiles.push(tile);
                    }
                }
            }

            let input = UserInput {
                hand_tiles: hand_tiles.clone(),
                open_melds: self.open_melds.clone(),
                closed_kans: self.closed_kans.clone(),
                winning_tile,
                agari_type: self.agari_type,
                player_context: PlayerContext {
                    jikaze: self.jikaze,
                    is_oya: self.jikaze == Kaze::Ton,
                    is_riichi: self.is_riichi,
                    is_daburu_riichi: self.is_daburu_riichi,
                    is_ippatsu: self.is_ippatsu,
                    is_menzen: self.open_melds.is_empty(),
                },
                game_context: GameContext {
                    bakaze: self.bakaze,
                    honba: self.honba,
                    dora_indicators: self.dora_indicators.clone(),
                    uradora_indicators: self.uradora_indicators.clone(),
                    num_akadora: self.num_akadora,
                    is_tenhou: self.is_tenhou,
                    is_chiihou: self.is_chiihou,
                    is_renhou: self.is_renhou,
                    is_haitei: self.is_haitei,
                    is_houtei: self.is_houtei,
                    is_rinshan: self.is_rinshan,
                    is_chankan: self.is_chankan,
                },
            };

            let mut best_result = calculate_agari(&input);

            // Winning Tile in Open Meld
            if best_result.is_err() {
                let base_open_melds = self.open_melds.clone();

                for (i, meld) in base_open_melds.iter().enumerate() {
                    let meld_tiles = self.get_meld_tiles(meld);
                    if meld_tiles.contains(&winning_tile) {
                        let mut alt_hand_tiles = hand_tiles.clone();
                        alt_hand_tiles.extend(meld_tiles.iter());

                        if let Some(pos) = alt_hand_tiles.iter().position(|x| *x == winning_tile) {
                            alt_hand_tiles.remove(pos);
                        }

                        let mut alt_open_melds = base_open_melds.clone();
                        alt_open_melds.remove(i);

                        let alt_input = UserInput {
                            hand_tiles: alt_hand_tiles,
                            open_melds: alt_open_melds,
                            winning_tile,
                            closed_kans: self.closed_kans.clone(),
                            agari_type: self.agari_type,
                            player_context: input.player_context.clone(),
                            game_context: input.game_context.clone(),
                        };

                        if let Ok(res) = calculate_agari(&alt_input) {
                            best_result = Ok(res);
                            break;
                        }
                    }
                }
            }

            self.score_result = match best_result {
                Ok(result) => Some(Ok(result)),
                Err(e) => Some(Err(format!("Error: {}", e))),
            };
            self.phase = super::Phase::Result;
        }
    }

    pub fn get_max_akadora_count(&self) -> u8 {
        let mut count_5m = 0;
        let mut count_5p = 0;
        let mut count_5s = 0;

        let check_tile = |tile: &Hai, c_m: &mut u8, c_p: &mut u8, c_s: &mut u8| {
            if let Hai::Suhai(Suhai { number: 5, suit }) = tile {
                match suit {
                    crate::implements::types::tiles::Suit::Manzu => *c_m += 1,
                    crate::implements::types::tiles::Suit::Pinzu => *c_p += 1,
                    crate::implements::types::tiles::Suit::Souzu => *c_s += 1,
                }
            }
        };

        for tile in &self.hand_tiles {
            check_tile(tile, &mut count_5m, &mut count_5p, &mut count_5s);
        }

        if let Some(tile) = &self.winning_tile {
            check_tile(tile, &mut count_5m, &mut count_5p, &mut count_5s);
        }

        for meld in &self.open_melds {
            for tile in self.get_meld_tiles(meld) {
                check_tile(&tile, &mut count_5m, &mut count_5p, &mut count_5s);
            }
        }

        for tile in &self.closed_kans {
            for _ in 0..4 {
                check_tile(tile, &mut count_5m, &mut count_5p, &mut count_5s);
            }
        }

        // 1 red 5-man, 2 red 5-pin, 1 red 5-sou
        let max_m = if count_5m > 0 { 1 } else { 0 };
        let max_p = if count_5p >= 2 { 2 } else { count_5p };
        let max_s = if count_5s > 0 { 1 } else { 0 };

        max_m + max_p + max_s
    }
}
