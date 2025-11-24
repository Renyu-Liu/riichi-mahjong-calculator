use crate::implements::game::AgariType;
use crate::implements::hand::MentsuType;
use crate::implements::input::OpenMeldInput;
use crate::implements::tiles::{Hai, Kaze};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    Composition,
    Definition,
    SelectingWinningTile,
    SelectingMeldTile(MentsuType, Option<usize>),
    SelectingClosedKan { editing_index: Option<usize> },
    SelectingDora,
    SelectingUraDora,
    Result,
}

pub struct RiichiGui {
    pub phase: Phase,

    // Phase 1 State
    pub hand_tiles: Vec<Hai>,
    pub tile_counts: [u8; 34],

    // Phase 2 State
    pub winning_tile: Option<Hai>,
    pub open_melds: Vec<OpenMeldInput>,
    pub closed_kans: Vec<Hai>,

    // Phase 3 State
    pub agari_type: AgariType,
    pub bakaze: Kaze,
    pub jikaze: Kaze,
    pub is_riichi: bool,
    pub is_daburu_riichi: bool,
    pub is_ippatsu: bool,
    pub is_rinshan: bool,
    pub is_chankan: bool,
    pub is_haitei: bool,
    pub is_houtei: bool,
    pub is_tenhou: bool,
    pub is_chiihou: bool,
    pub is_renhou: bool,
    pub honba: u8,
    pub num_akadora: u8,
    pub dora_indicators: Vec<Hai>,
    pub uradora_indicators: Vec<Hai>,

    pub score_result: Option<Result<crate::implements::scoring::AgariResult, String>>,
}

impl RiichiGui {
    pub fn new() -> Self {
        Self {
            phase: Phase::Composition,
            hand_tiles: Vec::new(),
            tile_counts: [4; 34],
            winning_tile: None,
            open_melds: Vec::new(),
            closed_kans: Vec::new(),
            agari_type: AgariType::Ron,
            bakaze: Kaze::Ton,
            jikaze: Kaze::Ton,
            is_riichi: false,
            is_daburu_riichi: false,
            is_ippatsu: false,
            is_rinshan: false,
            is_chankan: false,
            is_haitei: false,
            is_houtei: false,
            is_tenhou: false,
            is_chiihou: false,
            is_renhou: false,
            honba: 0,
            num_akadora: 0,
            dora_indicators: Vec::new(),
            uradora_indicators: Vec::new(),
            score_result: None,
        }
    }

    pub fn get_meld_tiles(&self, meld: &OpenMeldInput) -> Vec<Hai> {
        let mut tiles = Vec::new();
        match meld.mentsu_type {
            MentsuType::Shuntsu => {
                let start_idx = crate::implements::tiles::tile_to_index(&meld.representative_tile);
                if start_idx < 27 {
                    let suit_base = (start_idx / 9) * 9;

                    for i in 0..3 {
                        let idx = start_idx + i;
                        if idx < suit_base + 9 {
                            tiles.push(crate::implements::tiles::index_to_tile(idx));
                        }
                    }
                } else {
                    tiles.push(meld.representative_tile);
                }
            }
            MentsuType::Koutsu => {
                for _ in 0..3 {
                    tiles.push(meld.representative_tile);
                }
            }
            MentsuType::Kantsu => {
                for _ in 0..4 {
                    tiles.push(meld.representative_tile);
                }
            }
        }
        tiles
    }

    /// Counts the total number of 5-tiles in the hand (including winning tile, melds, and kans)
    pub fn count_five_tiles(&self) -> u8 {
        let mut count = 0;

        // Count 5-tiles in hand_tiles
        for tile in &self.hand_tiles {
            if matches!(tile, Hai::Suhai(5, _)) {
                count += 1;
            }
        }

        // Count 5-tiles in winning_tile
        if let Some(tile) = &self.winning_tile {
            if matches!(tile, Hai::Suhai(5, _)) {
                count += 1;
            }
        }

        // Count 5-tiles in open_melds
        for meld in &self.open_melds {
            let tiles = self.get_meld_tiles(meld);
            for tile in tiles {
                if matches!(tile, Hai::Suhai(5, _)) {
                    count += 1;
                }
            }
        }

        // Count 5-tiles in closed_kans
        for tile in &self.closed_kans {
            if matches!(tile, Hai::Suhai(5, _)) {
                // Closed kan has 4 tiles
                count += 4;
            }
        }

        count
    }
}
