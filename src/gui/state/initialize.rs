use super::RiichiGui;
use super::phase::Phase;
use crate::implements::types::{
    game::AgariType,
    input::OpenMeldInput,
    tiles::{Hai, Kaze},
};

#[derive(Default)]
struct GameStateDefaults {
    phase: Phase,
    hand_tiles: Vec<Hai>,
    winning_tile: Option<Hai>,
    open_melds: Vec<OpenMeldInput>,
    closed_kans: Vec<Hai>,
    agari_type: AgariType,
    bakaze: Kaze,
    jikaze: Kaze,
    is_riichi: bool,
    is_daburu_riichi: bool,
    is_ippatsu: bool,
    is_rinshan: bool,
    is_chankan: bool,
    is_haitei: bool,
    is_houtei: bool,
    is_tenhou: bool,
    is_chiihou: bool,
    is_renhou: bool,
    honba: u8,
    num_akadora: u8,
    dora_indicators: Vec<Hai>,
    uradora_indicators: Vec<Hai>,
    score_result: Option<Result<crate::implements::types::scoring::AgariResult, String>>,
    show_rules: bool,
}

impl RiichiGui {
    pub fn new() -> Self {
        use rayon::prelude::*;

        // image loading and resizing
        let (tile_images, tile_images_sideways): (
            std::collections::HashMap<Hai, iced::widget::image::Handle>,
            std::collections::HashMap<Hai, iced::widget::image::Handle>,
        ) = (0..34)
            .into_par_iter()
            .map(|i| {
                let tile = crate::implements::types::tiles::index_to_tile(i);
                let path = crate::gui::components::get_tile_image_path(&tile);

                // fast render
                if let Ok(img) = image::open(&path) {
                    let resized = img.resize(256, 256, image::imageops::FilterType::Nearest);
                    let rgba = resized.to_rgba8();
                    let width = rgba.width();
                    let height = rgba.height();
                    let pixels = rgba.into_raw();

                    let handle =
                        iced::widget::image::Handle::from_pixels(width, height, pixels.clone());

                    // sideways image
                    let rotated = resized.rotate270();
                    let rgba_rotated = rotated.to_rgba8();
                    let width_rotated = rgba_rotated.width();
                    let height_rotated = rgba_rotated.height();
                    let pixels_rotated = rgba_rotated.into_raw();
                    let handle_sideways = iced::widget::image::Handle::from_pixels(
                        width_rotated,
                        height_rotated,
                        pixels_rotated,
                    );

                    Some(((tile, handle), (tile, handle_sideways)))
                } else {
                    eprintln!("Failed to load image: {}", path);
                    None
                }
            })
            .filter_map(|x| x)
            .unzip();

        let rules_image = if let Ok(img) = image::open("assets/riichi_rule.png") {
            let rgba = img.to_rgba8();
            let width = rgba.width();
            let height = rgba.height();
            let pixels = rgba.into_raw();
            Some(iced::widget::image::Handle::from_pixels(
                width, height, pixels,
            ))
        } else {
            eprintln!("Failed to load rules image");
            None
        };

        let tile_back_image = if let Ok(img) = image::open("assets/tiles/Back.png") {
            let resized = img.resize(256, 256, image::imageops::FilterType::Nearest);
            let rgba = resized.to_rgba8();
            let width = rgba.width();
            let height = rgba.height();
            let pixels = rgba.into_raw();
            Some(iced::widget::image::Handle::from_pixels(
                width, height, pixels,
            ))
        } else {
            eprintln!("Failed to load tile back image");
            None
        };

        let defaults = GameStateDefaults::default();

        Self {
            phase: defaults.phase,
            hand_tiles: defaults.hand_tiles,
            tile_counts: [4; 34],
            winning_tile: defaults.winning_tile,
            open_melds: defaults.open_melds,
            closed_kans: defaults.closed_kans,
            agari_type: defaults.agari_type,
            bakaze: defaults.bakaze,
            jikaze: defaults.jikaze,
            is_riichi: defaults.is_riichi,
            is_daburu_riichi: defaults.is_daburu_riichi,
            is_ippatsu: defaults.is_ippatsu,
            is_rinshan: defaults.is_rinshan,
            is_chankan: defaults.is_chankan,
            is_haitei: defaults.is_haitei,
            is_houtei: defaults.is_houtei,
            is_tenhou: defaults.is_tenhou,
            is_chiihou: defaults.is_chiihou,
            is_renhou: defaults.is_renhou,
            honba: defaults.honba,
            num_akadora: defaults.num_akadora,
            dora_indicators: defaults.dora_indicators,
            uradora_indicators: defaults.uradora_indicators,
            score_result: defaults.score_result,
            show_rules: defaults.show_rules,
            tile_images,
            tile_images_sideways,
            tile_back_image,
            rules_image,
        }
    }

    pub fn reset(&mut self) {
        let defaults = GameStateDefaults::default();
        self.phase = defaults.phase;
        self.hand_tiles = defaults.hand_tiles;
        self.tile_counts = [4; 34];
        self.winning_tile = defaults.winning_tile;
        self.open_melds = defaults.open_melds;
        self.closed_kans = defaults.closed_kans;
        self.agari_type = defaults.agari_type;
        self.bakaze = defaults.bakaze;
        self.jikaze = defaults.jikaze;
        self.is_riichi = defaults.is_riichi;
        self.is_daburu_riichi = defaults.is_daburu_riichi;
        self.is_ippatsu = defaults.is_ippatsu;
        self.is_rinshan = defaults.is_rinshan;
        self.is_chankan = defaults.is_chankan;
        self.is_haitei = defaults.is_haitei;
        self.is_houtei = defaults.is_houtei;
        self.is_tenhou = defaults.is_tenhou;
        self.is_chiihou = defaults.is_chiihou;
        self.is_renhou = defaults.is_renhou;
        self.honba = defaults.honba;
        self.num_akadora = defaults.num_akadora;
        self.dora_indicators = defaults.dora_indicators;
        self.uradora_indicators = defaults.uradora_indicators;
        self.score_result = defaults.score_result;
        self.show_rules = defaults.show_rules;
    }
}
