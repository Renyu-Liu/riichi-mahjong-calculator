// yakuman.rs: yakuman checkers

use super::standard::check_chinitsu;
use super::utils::*;
use crate::implements::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{AgariHand, HandStructure, Machi},
    tiles::{Hai, Jihai, Suhai, index_to_tile, tile_to_index},
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

/// Kokushi Musou
pub fn check_kokushi(counts: &[u8; 34], agari_hai: Hai) -> Option<(HandStructure, Yaku)> {
    let mut has_pair = false;
    let mut tiles = Vec::new();
    let mut atama_tile = None;

    for (idx, &count) in counts.iter().enumerate() {
        let tile = index_to_tile(idx);
        if !tile.is_yaochuu() {
            if count > 0 {
                return None;
            }
            continue;
        }

        match count {
            1 => {
                tiles.push(tile);
            }
            2 => {
                if has_pair {
                    return None;
                }
                has_pair = true;
                atama_tile = Some(tile);
                tiles.push(tile);
            }
            0 => {}
            _ => return None,
        }
    }

    if !has_pair {
        return None;
    }

    let agari_hai_index = tile_to_index(&agari_hai);
    if counts[agari_hai_index] == 0 {
        return None;
    }

    let atama = (atama_tile.unwrap(), atama_tile.unwrap());

    // 13-sided wait
    let mut yaku = Yaku::KokushiMusou;
    let mut final_machi = Machi::KokushiIchimen;

    if atama.0 == agari_hai {
        if tiles.len() == 13 {
            yaku = Yaku::KokushiMusouJusanmen;
            final_machi = Machi::KokushiJusanmen;
        }
    }

    Some((
        HandStructure::KokushiMusou {
            tiles: tiles.try_into().ok()?,
            atama,
            _agari_hai: agari_hai,
            _machi: final_machi,
        },
        yaku,
    ))
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

/// Chuuren Poutou
fn check_chuuren(hand: &AgariHand) -> Option<bool> {
    let all_tiles = get_all_tiles(hand);

    let (is_chinitsu, suit) = check_chinitsu(&all_tiles);
    if !is_chinitsu {
        return None;
    }
    let suit = suit.unwrap();

    if !hand.mentsu.iter().all(|m| !m.is_minchou) {
        return None;
    }

    let mut counts = [0u8; 9];
    for tile in &all_tiles {
        if let Hai::Suhai(Suhai { number: n, suit: s }) = tile {
            if *s == suit {
                counts[(n - 1) as usize] += 1;
            }
        }
    }

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
    if let Hai::Suhai(Suhai { number: n, suit: s }) = hand.agari_hai {
        if s == suit && n as usize == extra_tile_num {
            return Some(true);
        }
    }

    Some(false)
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
