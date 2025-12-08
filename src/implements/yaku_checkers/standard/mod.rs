pub mod color;
pub mod ittsu;
pub mod peikou;
pub mod pinfu;
pub mod sanshoku;
pub mod tanyao;
pub mod terminals_honors;
pub mod yakuhai;

use self::{
    color::{check_chinitsu, check_honitsu},
    ittsu::check_ittsu,
    peikou::check_peikou,
    pinfu::check_pinfu,
    sanshoku::{check_sanshoku_doujun, check_sanshoku_doukou},
    tanyao::check_tanyao,
    terminals_honors::{check_chanta_junchan, check_shousangen},
    yakuhai::check_yakuhai,
};
use super::utils::*;
use crate::implements::types::{
    game::{AgariType, GameContext, PlayerContext},
    hand::{AgariHand, Machi, Mentsu, MentsuType},
    tiles::Hai,
    yaku::Yaku,
};

pub fn find_standard_yaku(
    hand: &AgariHand,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Vec<Yaku> {
    let mut yaku_list = Vec::new();

    // context-based Yaku
    if player.is_daburu_riichi {
        yaku_list.push(Yaku::DaburuRiichi);
    } else if player.is_riichi {
        yaku_list.push(Yaku::Riichi);
    }
    if player.is_ippatsu {
        yaku_list.push(Yaku::Ippatsu);
    }
    if player.is_menzen && agari_type == AgariType::Tsumo {
        yaku_list.push(Yaku::MenzenTsumo);
    }
    if game.is_haitei && agari_type == AgariType::Tsumo {
        yaku_list.push(Yaku::HaiteiRaoyue);
    }
    if game.is_houtei && agari_type == AgariType::Ron {
        yaku_list.push(Yaku::HouteiRaoyui);
    }
    if game.is_rinshan {
        yaku_list.push(Yaku::RinshanKaihou);
    }
    if game.is_chankan {
        yaku_list.push(Yaku::Chankan);
    }

    // Yakuhai
    yaku_list.extend(check_yakuhai(hand, player, game));

    // Pinfu
    if check_pinfu(hand, player, game) {
        yaku_list.push(Yaku::Pinfu);
    }

    // Tanyao
    if check_tanyao(hand) {
        yaku_list.push(Yaku::Tanyao);
    }

    // Sequence Yaku
    let shuntsu: Vec<&Mentsu> = hand
        .mentsu
        .iter()
        .filter(|m| m.mentsu_type == MentsuType::Shuntsu)
        .collect();

    if player.is_menzen {
        let (iipeikou, ryanpeikou) = check_peikou(&shuntsu);
        if ryanpeikou {
            yaku_list.push(Yaku::Ryanpeikou);
        } else if iipeikou {
            yaku_list.push(Yaku::Iipeikou);
        }
    }

    if check_sanshoku_doujun(&shuntsu) {
        yaku_list.push(Yaku::SanshokuDoujun);
    }

    if check_ittsu(&shuntsu) {
        yaku_list.push(Yaku::Ittsu);
    }

    // Triplet Yaku
    let (koutsu, kantsu) = count_koutsu_kantsu(hand);

    if koutsu + kantsu == 4 {
        yaku_list.push(Yaku::Toitoi);
    }

    let concealed_koutsu = count_concealed_koutsu(hand, agari_type);
    if concealed_koutsu == 3 {
        yaku_list.push(Yaku::Sanankou);
    }

    if kantsu == 3 {
        yaku_list.push(Yaku::Sankantsu);
    }

    if check_sanshoku_doukou(hand) {
        yaku_list.push(Yaku::SanshokuDoukou);
    }

    if check_shousangen(hand) {
        yaku_list.push(Yaku::Shousangen);
    }

    // Terminal/Honor Yaku
    let all_tiles = get_all_tiles(hand);
    let all_groups = get_all_groups(hand);

    let is_honroutou =
        all_tiles.iter().all(|t| t.is_yaochuu()) && !all_tiles.iter().all(|t| t.is_terminal()); // Exclude Chinroutou

    if is_honroutou {
        yaku_list.push(Yaku::Honroutou);
    } else {
        let (is_chanta, is_junchan) = check_chanta_junchan(&all_groups);
        if is_junchan {
            yaku_list.push(Yaku::Junchan);
        } else if is_chanta {
            yaku_list.push(Yaku::Chanta);
        }
    }

    // Color Yaku
    let (is_chinitsu, _) = check_chinitsu(&all_tiles);
    if is_chinitsu {
        yaku_list.push(Yaku::Chinitsu);
    } else {
        let (is_honitsu, _) = check_honitsu(&all_tiles);
        if is_honitsu {
            yaku_list.push(Yaku::Honitsu);
        }
    }

    yaku_list
}

pub fn find_chiitoitsu_yaku(
    pairs: &[(Hai, Hai); 7],
    _agari_hai: &Hai,
    _machi: &Machi,
    player: &PlayerContext,
    game: &GameContext,
    agari_type: AgariType,
) -> Vec<Yaku> {
    let mut yaku_list = Vec::new();

    yaku_list.push(Yaku::Chiitoitsu);

    // Riichi/DaburuRiichi/Ippatsu
    if player.is_daburu_riichi {
        yaku_list.push(Yaku::DaburuRiichi);
    } else if player.is_riichi {
        yaku_list.push(Yaku::Riichi);
    }
    if player.is_ippatsu {
        yaku_list.push(Yaku::Ippatsu);
    }
    // MenzenTsumo
    if agari_type == AgariType::Tsumo {
        yaku_list.push(Yaku::MenzenTsumo);
    }
    // Haitei/Houtei
    if game.is_haitei && agari_type == AgariType::Tsumo {
        yaku_list.push(Yaku::HaiteiRaoyue);
    }
    if game.is_houtei && agari_type == AgariType::Ron {
        yaku_list.push(Yaku::HouteiRaoyui);
    }

    // Tile-based Yaku
    let all_tiles: Vec<Hai> = pairs.iter().flat_map(|&(t1, t2)| vec![t1, t2]).collect();

    if all_tiles.iter().all(|t| t.is_simple()) {
        yaku_list.push(Yaku::Tanyao);
    }

    // Honroutou
    if all_tiles.iter().all(|t| t.is_yaochuu()) {
        yaku_list.push(Yaku::Honroutou);
    }

    // Color Yaku
    let (is_chinitsu, _) = check_chinitsu(&all_tiles);
    if is_chinitsu {
        yaku_list.push(Yaku::Chinitsu);
    } else {
        let (is_honitsu, _) = check_honitsu(&all_tiles);
        if is_honitsu {
            yaku_list.push(Yaku::Honitsu);
        }
    }

    yaku_list
}
