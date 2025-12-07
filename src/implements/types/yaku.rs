#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Yaku {
    // 1 Han
    Riichi,           // 立直 (Riichi)
    Ippatsu,          // 一発 (Ippatsu)
    MenzenTsumo,      // 門前清自摸和 (Fully Concealed Hand)
    Pinfu,            // 平和 (No-Points Hand)
    Iipeikou,         // 一盃口 (Pure Double Sequence)
    HaiteiRaoyue,     // 海底撈月 (Under the Sea)
    HouteiRaoyui,     // 河底撈魚 (Under the River)
    RinshanKaihou,    // 嶺上開花 (After a Kan)
    Chankan,          // 搶槓 (Robbing a Kan)
    Tanyao,           // 断幺九 (All Simples)
    YakuhaiJikaze,    // 役牌: 自風 (Seat Wind)
    YakuhaiBakaze,    // 役牌: 場風 (Prevalent Wind)
    YakuhaiSangenpai, // 役牌: 三元牌 (Dragon)

    // 2 Han
    DaburuRiichi,   // ダブル立直 (Double Riichi)
    Chiitoitsu,     // 七対子 (Seven Pairs)
    SanshokuDoujun, // 三色同順 (Mixed Triple Sequence) kuisagari
    Ittsu,          // 一気通貫 (Pure Straight) kuisagari
    Chanta,         // 全帯幺九 (Half Outside Hand) kuisagari
    Toitoi,         // 対々和 (All Triplets)
    Sanankou,       // 三暗刻 (Three Concealed Triplets)
    SanshokuDoukou, // 三色同刻 (Triple Triplets)
    Sankantsu,      // 三槓子 (Three Quads)
    Shousangen,     // 小三元 (Little Three Dragons)
    Honroutou,      // 混老頭 (All Terminals and Honors)

    // 3 Han
    Ryanpeikou, // 二盃口 (Twice Pure Double Sequence)
    Junchan,    // 純全帯么 (Fully Outside Hand) kuisagari
    Honitsu,    // 混一色 (Half Flush) kuisagari

    // 6 Han
    Chinitsu, // 清一色 (Full Flush) kuisagari

    // Yakuman (13 Han)
    Tenhou,               // 天和 (Blessing of Heaven)
    Chiihou,              // 地和 (Blessing of Earth)
    Renhou,               // 人和 (Blessing of Man)
    Daisangen,            // 大三元 (Big Three Dragons)
    Suuankou,             // 四暗刻 (Four Concealed Triplets)
    Daisuushi,            // 大四喜 (Four Big Winds)
    Shousuushi,           // 小四喜 (Four Little Winds)
    Tsuuiisou,            // 字一色 (All Honors)
    Chinroutou,           // 清老頭 (All Terminals)
    Ryuuiisou,            // 緑一色 (All Green)
    Suukantsu,            // 四槓子 (Four Quads)
    KokushiMusou,         // 国士無双 (Thirteen Orphans)
    ChuurenPoutou,        // 九蓮宝燈 (Nine Gates)
    SuuankouTanki,        // 四暗刻単騎 (Single Wait Four Concealed)
    KokushiMusouJusanmen, // 国士無S双13面待ち (13-Sided Wait Kokushi)
    JunseiChuurenPoutou,  // 純正九蓮宝燈 (True Nine Gates)

    // Dora (not Yaku)
    Dora,    // ドラ (Dora)
    UraDora, // 裏ドラ (Ura Dora)
    AkaDora, // 赤ドラ (Red Five Dora)
}

impl std::fmt::Display for Yaku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Yaku::Riichi => "Riichi",
            Yaku::Ippatsu => "Ippatsu",
            Yaku::MenzenTsumo => "Menzenchin Tsumohou",
            Yaku::Pinfu => "Pinfu",
            Yaku::Iipeikou => "Iipeikou",
            Yaku::HaiteiRaoyue => "Haitei Raoyue",
            Yaku::HouteiRaoyui => "Houtei Raoyui",
            Yaku::RinshanKaihou => "Rinshan Kaihou",
            Yaku::Chankan => "Chankan",
            Yaku::Tanyao => "Tanyao",
            Yaku::YakuhaiJikaze => "Yakuhai: Jikaze",
            Yaku::YakuhaiBakaze => "Yakuhai: Bakaze",
            Yaku::YakuhaiSangenpai => "Yakuhai: Sangenpai",
            Yaku::DaburuRiichi => "Double Riichi",
            Yaku::Chiitoitsu => "Chiitoitsu",
            Yaku::SanshokuDoujun => "Sanshoku Doujun",
            Yaku::Ittsu => "Ittsu",
            Yaku::Chanta => "Chanta",
            Yaku::Toitoi => "Toitoi",
            Yaku::Sanankou => "Sanankou",
            Yaku::SanshokuDoukou => "Sanshoku Doukou",
            Yaku::Sankantsu => "Sankantsu",
            Yaku::Shousangen => "Shousangen",
            Yaku::Honroutou => "Honroutou",
            Yaku::Ryanpeikou => "Ryanpeikou",
            Yaku::Junchan => "Junchan",
            Yaku::Honitsu => "Honitsu",
            Yaku::Chinitsu => "Chinitsu",
            Yaku::Tenhou => "Tenhou",
            Yaku::Chiihou => "Chiihou",
            Yaku::Renhou => "Renhou",
            Yaku::Daisangen => "Daisangen",
            Yaku::Suuankou => "Suuankou",
            Yaku::Daisuushi => "Daisuushi",
            Yaku::Shousuushi => "Shousuushi",
            Yaku::Chinroutou => "Chinroutou",
            Yaku::Ryuuiisou => "Ryuuiisou",
            Yaku::Tsuuiisou => "Tsuuiisou",
            Yaku::Suukantsu => "Suukantsu",
            Yaku::KokushiMusou => "Kokushi Musou",
            Yaku::ChuurenPoutou => "Chuuren Poutou",
            Yaku::SuuankouTanki => "Suuankou Tanki",
            Yaku::KokushiMusouJusanmen => "Kokushi Musou 13-Wait",
            Yaku::JunseiChuurenPoutou => "Junsei Chuuren Poutou",
            Yaku::Dora => "Dora",
            Yaku::UraDora => "Ura Dora",
            Yaku::AkaDora => "Aka Dora",
        };
        write!(f, "{}", name)
    }
}
