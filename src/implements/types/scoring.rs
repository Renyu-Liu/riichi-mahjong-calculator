use super::game::AgariType;
use super::yaku::Yaku;

// fu calculation
pub const FU_CHIITOITSU: u8 = 25;
pub const FU_PINFU_TSUMO: u8 = 20;
pub const FU_PINFU_RON: u8 = 30;
pub const FU_BASE: u8 = 20;
pub const FU_MENZEN_RON: u8 = 10;
pub const FU_TSUMO: u8 = 2;
pub const FU_PAIR_SINGLE_WAIT: u8 = 2;
pub const FU_PAIR_WIND: u32 = 2;
pub const FU_PAIR_DRAGON: u32 = 2;
pub const FU_ROUND_UP: u8 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
// point limits
pub enum HandLimit {
    Mangan,    // 満貫
    Haneman,   // 跳満
    Baiman,    // 倍満
    Sanbaiman, // 三倍満
    Yakuman,   // 役満 (13han+)
}

#[derive(Debug, Clone)]
// result for a winning hand
pub struct AgariResult {
    pub han: u8,              // 飜 (Han count)
    pub fu: u8,               // 符 (Fu count)
    pub yaku_list: Vec<Yaku>, // all yaku and dora achieved
    pub num_akadora: u8,      // 赤ドラ (Red Dora)
    pub limit_name: Option<HandLimit>,
    pub oya_payment: u32,
    pub ko_payment: u32,
    pub total_payment: u32,
    pub honba: u8,
    pub agari_type: AgariType,
    pub is_oya: bool,
}
