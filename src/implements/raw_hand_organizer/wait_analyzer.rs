use crate::implements::types::{
    hand::{Machi, Mentsu, MentsuType},
    tiles::{Hai, tile_to_index},
};

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
) -> Vec<(Machi, usize)> {
    let mut possible_waits = Vec::new();

    if agari_hai == atama.0 {
        possible_waits.push((Machi::Tanki, 4));
    }

    for (i, winning_meld) in mentsu.iter().enumerate() {
        if !mentsu_contains_tile(winning_meld, &agari_hai) {
            continue;
        }

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
        possible_waits.push((machi, i));
    }
    possible_waits
}
