// raw_hand_organizer.rs: Organizes a raw hand input into standard melds and pair

use super::types::{
    game::AgariType,
    hand::{AgariHand, HandOrganization, Machi, Mentsu, MentsuType},
    input::UserInput,
    tiles::{Hai, index_to_tile, tile_to_index},
};
use std::convert::TryInto;

// find melds
mod recursive_parser {
    use super::*;

    pub fn find_all_mentsu_recursive(
        counts: &mut [u8; 34],
        mentsu: &mut Vec<Mentsu>,
        results: &mut Vec<Vec<Mentsu>>,
    ) {
        let mut i = 0;
        while i < 34 && counts[i] == 0 {
            i += 1;
        }
        if i == 34 {
            // Found a valid combination
            results.push(mentsu.clone());
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

            find_all_mentsu_recursive(counts, mentsu, results);

            mentsu.pop();
            counts[i] += 3;
        }

        // Find Shuntsu
        if i < 27 && (i % 9) < 7 && counts[i] > 0 && counts[i + 1] > 0 && counts[i + 2] > 0 {
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

            find_all_mentsu_recursive(counts, mentsu, results);

            mentsu.pop();
            counts[i] += 1;
            counts[i + 1] += 1;
            counts[i + 2] += 1;
        }
    }
}

// Wait Type
mod wait_analyzer {
    use super::*;

    fn mentsu_contains_tile(mentsu: &Mentsu, tile: &Hai) -> bool {
        match mentsu.mentsu_type {
            MentsuType::Koutsu | MentsuType::Kantsu => mentsu.tiles[0] == *tile,
            MentsuType::Shuntsu => {
                mentsu.tiles[0] == *tile || mentsu.tiles[1] == *tile || mentsu.tiles[2] == *tile
            }
        }
    }

    pub fn determine_wait_type(
        mentsu: &[Mentsu; 4],
        atama: (Hai, Hai),
        agari_hai: Hai,
    ) -> Vec<Machi> {
        let mut possible_waits = Vec::new();

        if agari_hai == atama.0 {
            possible_waits.push(Machi::Tanki);
        }

        let winning_melds: Vec<&Mentsu> = mentsu
            .iter()
            .filter(|m| mentsu_contains_tile(m, &agari_hai))
            .collect();

        for winning_meld in winning_melds {
            let machi = match winning_meld.mentsu_type {
                MentsuType::Koutsu | MentsuType::Kantsu => Machi::Shanpon,
                MentsuType::Shuntsu => {
                    let t1 = winning_meld.tiles[0];
                    let t2 = winning_meld.tiles[1];
                    let t3 = winning_meld.tiles[2];

                    if agari_hai == t2 {
                        Machi::Kanchan
                    } else if agari_hai == t1 {
                        if tile_to_index(&t3) % 9 == 8 {
                            Machi::Penchan
                        } else {
                            Machi::Ryanmen
                        }
                    } else if agari_hai == t3 {
                        if tile_to_index(&t1) % 9 == 0 {
                            Machi::Penchan
                        } else {
                            Machi::Ryanmen
                        }
                    } else {
                        continue;
                    }
                }
            };
            possible_waits.push(machi);
        }
        possible_waits
    }
}

pub fn organize_hand(input: &UserInput) -> Result<Vec<HandOrganization>, &'static str> {
    let mut master_counts = [0u8; 34];
    for tile in &input.hand_tiles {
        master_counts[tile_to_index(tile)] += 1;
    }

    if input.agari_type == AgariType::Ron {
        master_counts[tile_to_index(&input.winning_tile)] += 1;
    }

    let concealed_counts = master_counts;
    let mut final_results: Vec<HandOrganization> = Vec::new();
    let mut open_mentsu: Vec<Mentsu> = Vec::with_capacity(4);

    // Closed Kans
    for rep_tile in &input.closed_kans {
        let kan_tile = *rep_tile;
        open_mentsu.push(Mentsu {
            mentsu_type: MentsuType::Kantsu,
            is_minchou: false,
            tiles: [kan_tile, kan_tile, kan_tile, kan_tile],
        });
    }

    // Open Melds
    for meld in &input.open_melds {
        let rep_tile = meld.representative_tile;
        let index = tile_to_index(&rep_tile);

        match meld.mentsu_type {
            MentsuType::Koutsu => {
                open_mentsu.push(Mentsu {
                    mentsu_type: MentsuType::Koutsu,
                    is_minchou: true,
                    tiles: [rep_tile, rep_tile, rep_tile, rep_tile],
                });
            }
            MentsuType::Kantsu => {
                open_mentsu.push(Mentsu {
                    mentsu_type: MentsuType::Kantsu,
                    is_minchou: true,
                    tiles: [rep_tile, rep_tile, rep_tile, rep_tile],
                });
            }
            MentsuType::Shuntsu => {
                let index1 = index;
                let index2 = index1 + 1;
                let index3 = index1 + 2;

                if index1 >= 27 || (index1 % 9) >= 7 {
                    return Err("Invalid representative tile for Chi (must be 1-7)");
                }

                let t1 = rep_tile;
                let t2 = index_to_tile(index2);
                let t3 = index_to_tile(index3);
                open_mentsu.push(Mentsu {
                    mentsu_type: MentsuType::Shuntsu,
                    is_minchou: true,
                    tiles: [t1, t2, t3, t3],
                });
            }
        }
    }

    let mentsu_needed = 4 - open_mentsu.len();
    let agari_hai = input.winning_tile;

    // 4 known melds
    if mentsu_needed == 0 {
        for i in 0..34 {
            if concealed_counts[i] == 2 {
                let pair_tile = index_to_tile(i);
                let atama = (pair_tile, pair_tile);

                let mentsu_array: [Mentsu; 4] = open_mentsu
                    .clone()
                    .try_into()
                    .map_err(|_| "final_mentsu length not 4")?;

                let agari_hand = AgariHand {
                    mentsu: mentsu_array,
                    atama,
                    agari_hai,
                    machi: Machi::Tanki,
                };

                final_results.push(HandOrganization::YonmentsuIchiatama(agari_hand));
            }
        }
        if input.hand_tiles.len() == 14 {
        } else if final_results.is_empty() {
            return Err("4 open melds but no pair found");
        }
    } else {
        // Standard Hand
        for i in 0..34 {
            if concealed_counts[i] >= 2 {
                let mut temp_counts = concealed_counts;
                temp_counts[i] -= 2;
                let atama = (index_to_tile(i), index_to_tile(i));
                let mut closed_mentsu: Vec<Mentsu> = Vec::with_capacity(mentsu_needed);
                let mut recursive_results: Vec<Vec<Mentsu>> = Vec::new();

                recursive_parser::find_all_mentsu_recursive(
                    &mut temp_counts,
                    &mut closed_mentsu,
                    &mut recursive_results,
                );

                for res in recursive_results {
                    if res.len() == mentsu_needed {
                        let mut full_mentsu = open_mentsu.clone();
                        full_mentsu.extend(res);

                        let mentsu_array: [Mentsu; 4] = full_mentsu
                            .try_into()
                            .map_err(|_| "final_mentsu length not 4")?;

                        let possible_waits =
                            wait_analyzer::determine_wait_type(&mentsu_array, atama, agari_hai);

                        for machi in possible_waits {
                            let agari_hand = AgariHand {
                                mentsu: mentsu_array,
                                atama,
                                agari_hai,
                                machi,
                            };

                            final_results.push(HandOrganization::YonmentsuIchiatama(agari_hand));
                        }
                    }
                }
            }
        }
    }

    final_results.push(HandOrganization::Irregular {
        counts: master_counts,
        agari_hai,
    });

    Ok(final_results)
}
