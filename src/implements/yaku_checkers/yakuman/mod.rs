pub mod chuuren;
pub mod kokushi;

use self::chuuren::check_chuuren;
pub use self::kokushi::check_kokushi;

use super::utils::*;
use crate::implements::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{AgariHand, HandStructure, Machi},
    tiles::{Hai, Jihai},
    yaku::Yaku,
};

pub fn check_game_state_yakuman(_player: &PlayerContext, game: &GameContext) -> Vec<Yaku> {
    let mut yaku = Vec::new();
    if game.is_tenhou {
        yaku.push(Yaku::Tenhou);
    }
    if game.is_chiihou {
        yaku.push(Yaku::Chiihou);
    }
    if game.is_renhou {
        yaku.push(Yaku::Renhou);
    }
    yaku
}

pub fn check_standard_yakuman(
    hand: &AgariHand,
    _player: &PlayerContext,
    _game: &GameContext,
    agari_type: AgariType,
) -> (Vec<Yaku>, Option<bool>) {
    let mut yakuman = Vec::new();
    let all_tiles = get_all_tiles(hand);

    // Tile-based Yakuman
    let mut is_tsuuiisou = true;
    let mut is_chinroutou = true;
    let mut is_ryuuiisou = true;

    for tile in &all_tiles {
        if !tile.is_jihai() {
            is_tsuuiisou = false;
        }
        if !tile.is_terminal() {
            is_chinroutou = false;
        }
        if !is_green_tile(tile) {
            is_ryuuiisou = false;
        }
    }

    if is_tsuuiisou {
        yakuman.push(Yaku::Tsuuiisou);
    }
    if is_chinroutou {
        yakuman.push(Yaku::Chinroutou);
    }
    if is_ryuuiisou {
        yakuman.push(Yaku::Ryuuiisou);
    }

    //  Meld-based Yakuman
    let (_koutsu, kantsu) = count_koutsu_kantsu(hand);
    let concealed_koutsu = count_concealed_koutsu(hand, agari_type);

    // Suukantsu
    if kantsu == 4 {
        yakuman.push(Yaku::Suukantsu);
    }

    // Suuankou
    if concealed_koutsu == 4 {
        if hand.machi == Machi::Tanki {
            yakuman.push(Yaku::SuuankouTanki);
        } else {
            yakuman.push(Yaku::Suuankou);
        }
    }

    // Daisangen
    let mut dragon_koutsu = 0;
    for mentsu in &hand.mentsu {
        if is_koutsu_or_kantsu(mentsu) {
            if let Hai::Jihai(Jihai::Sangen(_)) = mentsu.tiles[0] {
                dragon_koutsu += 1;
            }
        }
    }
    if dragon_koutsu == 3 {
        yakuman.push(Yaku::Daisangen);
    }

    // Daisuushi / Shousuushi
    let mut wind_koutsu = 0;
    let mut wind_atama = false;
    for mentsu in &hand.mentsu {
        if is_koutsu_or_kantsu(mentsu) {
            if let Hai::Jihai(Jihai::Kaze(_)) = mentsu.tiles[0] {
                wind_koutsu += 1;
            }
        }
    }
    if let Hai::Jihai(Jihai::Kaze(_)) = hand.atama.0 {
        wind_atama = true;
    }

    if wind_koutsu == 4 {
        yakuman.push(Yaku::Daisuushi);
    } else if wind_koutsu == 3 && wind_atama {
        yakuman.push(Yaku::Shousuushi);
    }

    // Chuuren Poutou
    let chuuren_flag = check_chuuren(hand);
    if let Some(is_junsei) = chuuren_flag {
        if is_junsei {
            yakuman.push(Yaku::JunseiChuurenPoutou);
        } else {
            yakuman.push(Yaku::ChuurenPoutou);
        }
    }

    (yakuman, chuuren_flag)
}

/// Tsuuiisou
pub fn check_chiitoitsu_yakuman(hand: &HandStructure) -> Vec<Yaku> {
    if let HandStructure::Chiitoitsu { pairs, .. } = hand {
        let mut is_tsuuiisou = true;
        for (tile, _) in pairs {
            if !tile.is_jihai() {
                is_tsuuiisou = false;
                break;
            }
        }
        if is_tsuuiisou {
            return vec![Yaku::Tsuuiisou];
        }
    }
    vec![]
}

/// Double Yakuman overrides
pub fn post_process_yakuman(mut yakuman: Vec<Yaku>) -> Vec<Yaku> {
    let has_suuankou_tanki = yakuman.contains(&Yaku::SuuankouTanki);
    let has_kokushi_jusanmen = yakuman.contains(&Yaku::KokushiMusouJusanmen);
    let has_junsei_chuuren = yakuman.contains(&Yaku::JunseiChuurenPoutou);

    yakuman.retain(|&y| {
        (y != Yaku::Suuankou || !has_suuankou_tanki)
            && (y != Yaku::KokushiMusou || !has_kokushi_jusanmen)
            && (y != Yaku::ChuurenPoutou || !has_junsei_chuuren)
    });

    yakuman
}
