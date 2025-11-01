enum Suit { Man, Pin, Sou }
enum Honor { Wind(Wind), Dragon(Dragon) }
enum Wind { East, South, West, North }
enum Dragon { White, Green, Red }

enum Tile {
    Number(u8, Suit), // u8 from 1-9
    Honor(Honor),
}
struct Meld {
    meld_type: MeldType,
    is_open: bool,
    tiles: [Tile; 4], // Use 4, Kants just have a different type
}
enum MeldType { Sequence, Triplet, Kan }
// An array where the index 0-33 corresponds to a unique tile.
// e.g., 0-8=Man, 9-17=Pin, 18-26=Sou, 27-33=Honors
// The value at the index is the count of that tile (0-4).
struct Hand {
    tiles: [u8; 34],
}
struct WinningHand {
    melds: [Meld; 4],
    pair: (Tile, Tile),
    winning_tile: Tile,
    wait_type: WaitType, // e.g., TwoSided, Pair, Edge
}
struct GameConditions {
    seat_wind: Wind,
    prevalent_wind: Wind,
    is_riichi: bool,
    is_ippatsu: bool,
    is_tsumo: bool,
    is_closed_hand: bool,
    dora_indicators: Vec<Tile>,
    ura_dora_indicators: Vec<Tile>,
    // ... and booleans for Haitei, Rinshan, etc.
}

// An enum listing every Yaku, storing its Han value.
enum Yaku {
    Riichi(u8),       // 1 Han
    Tanyao(u8),       // 1 Han
    Pinfu(u8),       // 1 Han
    Honitsu(u8),     // 3 Han (2 if open)
    Daisangen(u8),        // Yakuman
    // ... etc.
}
// The final output of our library.
struct ScoreResult {
    han: u8,
    fu: u8,
    points: u32,
    yaku_list: Vec<Yaku>,
    score_name: String, // e.g., "Mangan", "Haneman"
}