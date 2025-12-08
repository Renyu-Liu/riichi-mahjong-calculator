use crate::implements::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{HandStructure, Machi, MentsuType},
    scoring::{
        FU_BASE, FU_CHIITOITSU, FU_MENZEN_RON, FU_PAIR_DRAGON, FU_PAIR_SINGLE_WAIT, FU_PAIR_WIND,
        FU_PINFU_RON, FU_PINFU_TSUMO, FU_ROUND_UP, FU_TSUMO,
    },
    tiles::{Hai, Jihai},
    yaku::Yaku,
};

const FU_TABLE_KOUTSU: [[u32; 2]; 2] = [
    [4, 8], // Closed: simple, terminal
    [2, 4], // Open: simple, terminal
];

const FU_TABLE_KANTSU: [[u32; 2]; 2] = [
    [16, 32], // Closed: simple, terminal
    [8, 16],  // Open: simple, terminal
];

pub fn calculate_fu(
    hand_structure: &HandStructure,
    yaku_list: &[Yaku],
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> u8 {
    // Chiitoitsu
    if yaku_list.contains(&Yaku::Chiitoitsu) {
        return FU_CHIITOITSU;
    }

    // Pinfu
    if yaku_list.contains(&Yaku::Pinfu) {
        return if agari_type == AgariType::Tsumo {
            FU_PINFU_TSUMO
        } else {
            FU_PINFU_RON
        };
    }

    // Standard Fu
    let mut fu = FU_BASE as u32;

    let hand = match hand_structure {
        HandStructure::YonmentsuIchiatama(h) => h,
        HandStructure::ChuurenPoutou { hand, .. } => hand,
        HandStructure::Chiitoitsu { .. } => return FU_CHIITOITSU,
        HandStructure::KokushiMusou { .. } => return 0,
    };

    // Agari Type
    if agari_type == AgariType::Tsumo {
        fu += FU_TSUMO as u32;
    } else if player.is_menzen {
        fu += FU_MENZEN_RON as u32;
    }

    // Melds
    for mentsu in &hand.mentsu {
        let is_open = mentsu.is_minchou;
        let is_yaochuu = mentsu.tiles[0].is_yaochuu();

        match mentsu.mentsu_type {
            MentsuType::Koutsu => {
                fu += FU_TABLE_KOUTSU[is_open as usize][is_yaochuu as usize];
            }
            MentsuType::Kantsu => {
                fu += FU_TABLE_KANTSU[is_open as usize][is_yaochuu as usize];
            }
            MentsuType::Shuntsu => {}
        }
    }

    // Pair
    fu += get_pair_fu(&hand.atama.0, player, game);

    // Wait
    match hand.machi {
        Machi::Kanchan | Machi::Penchan | Machi::Tanki => fu += FU_PAIR_SINGLE_WAIT as u32,
        _ => {}
    }

    let round_up = FU_ROUND_UP as u32;
    (((fu + round_up - 1) / round_up) * round_up) as u8
}

fn get_pair_fu(tile: &Hai, player: &PlayerContext, game: &GameContext) -> u32 {
    match tile {
        // Dragon Pair
        Hai::Jihai(Jihai::Sangen(_)) => FU_PAIR_DRAGON,
        // Wind Pair
        Hai::Jihai(Jihai::Kaze(k)) => {
            let mut fu = 0;
            if *k == game.bakaze {
                fu += FU_PAIR_WIND;
            }
            if *k == player.jikaze {
                fu += FU_PAIR_WIND;
            }
            fu
        }
        _ => 0,
    }
}
