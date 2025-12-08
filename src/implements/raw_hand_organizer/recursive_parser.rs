use crate::implements::types::{
    hand::{Mentsu, MentsuType},
    tiles::{MAX_SEQUENCE_START, SUIT_TILES_COUNT, TILE_COUNT, TILES_PER_SUIT, index_to_tile},
};

pub fn find_all_mentsu_recursive<F>(
    counts: &mut [u8; TILE_COUNT],
    mentsu: &mut Vec<Mentsu>,
    callback: &mut F,
) where
    F: FnMut(&Vec<Mentsu>),
{
    let mut i = 0;
    while i < TILE_COUNT && counts[i] == 0 {
        i += 1;
    }
    if i == TILE_COUNT {
        callback(mentsu);
        return;
    }

    // Find Koutsu
    if counts[i] >= 3 {
        let tile = index_to_tile(i);
        counts[i] -= 3;
        mentsu.push(Mentsu {
            mentsu_type: MentsuType::Koutsu,
            is_minchou: false,
            tiles: [tile, tile, tile, tile],
        });

        find_all_mentsu_recursive(counts, mentsu, callback);

        mentsu.pop();
        counts[i] += 3;
    }

    // Find Shuntsu
    if i < SUIT_TILES_COUNT
        && (i % TILES_PER_SUIT) < MAX_SEQUENCE_START
        && counts[i] > 0
        && counts[i + 1] > 0
        && counts[i + 2] > 0
    {
        let tile1 = index_to_tile(i);
        let tile2 = index_to_tile(i + 1);
        let tile3 = index_to_tile(i + 2);

        counts[i] -= 1;
        counts[i + 1] -= 1;
        counts[i + 2] -= 1;
        mentsu.push(Mentsu {
            mentsu_type: MentsuType::Shuntsu,
            is_minchou: false,
            tiles: [tile1, tile2, tile3, tile3],
        });

        find_all_mentsu_recursive(counts, mentsu, callback);

        mentsu.pop();
        counts[i] += 1;
        counts[i + 1] += 1;
        counts[i + 2] += 1;
    }
}
