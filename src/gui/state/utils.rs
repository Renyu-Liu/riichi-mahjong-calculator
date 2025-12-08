use super::super::state::RiichiGui;
use crate::implements::types::tiles::TILE_COUNT;

impl RiichiGui {
    /// available tiles
    pub fn get_active_hand_counts(&self) -> [u8; TILE_COUNT] {
        let mut counts = [0u8; TILE_COUNT];

        for tile in &self.hand_tiles {
            counts[crate::implements::types::tiles::tile_to_index(tile)] += 1;
        }

        for meld in &self.open_melds {
            for tile in self.get_meld_tiles(meld) {
                let idx = crate::implements::types::tiles::tile_to_index(&tile);
                if counts[idx] > 0 {
                    counts[idx] -= 1;
                }
            }
        }

        for tile in &self.closed_kans {
            let idx = crate::implements::types::tiles::tile_to_index(tile);
            if counts[idx] >= 4 {
                counts[idx] -= 4;
            } else {
                counts[idx] = 0;
            }
        }

        counts
    }
}
