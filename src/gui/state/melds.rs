use super::RiichiGui;
use crate::implements::types::{
    hand::MentsuType,
    input::OpenMeldInput,
    tiles::{
        Hai, MAX_SEQUENCE_START, SUIT_TILES_COUNT, TILE_COUNT, TILES_PER_SUIT, index_to_tile,
        tile_to_index,
    },
};

impl RiichiGui {
    pub fn get_meld_tiles(&self, meld: &OpenMeldInput) -> Vec<Hai> {
        let mut tiles = Vec::new();
        match meld.mentsu_type {
            MentsuType::Shuntsu => {
                let start_idx = tile_to_index(&meld.representative_tile);
                if start_idx < SUIT_TILES_COUNT {
                    let suit_base = (start_idx / TILES_PER_SUIT) * TILES_PER_SUIT;

                    for i in 0..3 {
                        let idx = start_idx + i;
                        if idx < suit_base + TILES_PER_SUIT {
                            tiles.push(index_to_tile(idx));
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

    /// meld availability
    pub fn can_form_meld(&self, meld: &OpenMeldInput) -> bool {
        let mut hand_counts = self.get_active_hand_counts();

        for tile in self.get_meld_tiles(meld) {
            let idx = tile_to_index(&tile);
            if hand_counts[idx] > 0 {
                hand_counts[idx] -= 1;
            } else {
                return false;
            }
        }

        true
    }

    pub fn get_all_possible_pons(&self) -> Vec<OpenMeldInput> {
        let available_counts = self.get_active_hand_counts();

        let mut pons = Vec::new();
        for i in 0..TILE_COUNT {
            if available_counts[i] >= 3 {
                let tile = index_to_tile(i);
                pons.push(OpenMeldInput {
                    mentsu_type: MentsuType::Koutsu,
                    representative_tile: tile,
                    is_added_kan: false,
                });
            }
        }
        pons
    }

    pub fn get_all_possible_chiis(&self) -> Vec<OpenMeldInput> {
        let available_counts = self.get_active_hand_counts();

        let mut chiis = Vec::new();
        for suit_offset in [0, TILES_PER_SUIT, TILES_PER_SUIT * 2] {
            for start_num in 0..MAX_SEQUENCE_START {
                let idx1 = suit_offset + start_num;
                let idx2 = idx1 + 1;
                let idx3 = idx1 + 2;

                if available_counts[idx1] > 0
                    && available_counts[idx2] > 0
                    && available_counts[idx3] > 0
                {
                    let tile = index_to_tile(idx1);
                    chiis.push(OpenMeldInput {
                        mentsu_type: MentsuType::Shuntsu,
                        representative_tile: tile,
                        is_added_kan: false,
                    });
                }
            }
        }
        chiis
    }

    pub fn get_all_possible_kans(&self) -> Vec<Hai> {
        let available_counts = self.get_active_hand_counts();

        let mut kans = Vec::new();
        for i in 0..TILE_COUNT {
            if available_counts[i] == 4 {
                let tile = index_to_tile(i);
                kans.push(tile);
            }
        }
        kans
    }
}
