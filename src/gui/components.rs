use super::messages::Message;
use super::styles::ColoredButtonStyle;
use crate::implements::tiles::{Hai, Jihai, Kaze, Sangenpai, Suhai, Suit};
use iced::Element;
use iced::theme;
use iced::widget::{button, column, row, text};

#[allow(dead_code)]
pub trait OnPressMaybe {
    fn on_press_maybe(self, msg: Option<Message>) -> Self;
}

impl<'a> OnPressMaybe for button::Button<'a, Message> {
    fn on_press_maybe(self, msg: Option<Message>) -> Self {
        match msg {
            Some(m) => self.on_press(m),
            None => self,
        }
    }
}

pub fn get_tile_image_path(tile: &Hai) -> String {
    let filename = match tile {
        Hai::Suhai(Suhai { number, suit }) => {
            let suit_prefix = match suit {
                Suit::Manzu => "Man",
                Suit::Pinzu => "Pin",
                Suit::Souzu => "Sou",
            };
            format!("{}{}.png", suit_prefix, number)
        }
        Hai::Jihai(jihai) => {
            let name = match jihai {
                Jihai::Kaze(Kaze::Ton) => "Ton",
                Jihai::Kaze(Kaze::Nan) => "Nan",
                Jihai::Kaze(Kaze::Shaa) => "Shaa",
                Jihai::Kaze(Kaze::Pei) => "Pei",
                Jihai::Sangen(Sangenpai::Haku) => "Haku",
                Jihai::Sangen(Sangenpai::Hatsu) => "Hatsu",
                Jihai::Sangen(Sangenpai::Chun) => "Chun",
            };
            format!("{}.png", name)
        }
    };
    format!("assets/tiles/{}", filename)
}

/// tile groups
pub fn create_grid(elements: Vec<Element<Message>>, columns: usize) -> Element<Message> {
    let mut grid_rows: Vec<Element<Message>> = Vec::new();
    let mut current_row_elements: Vec<Element<Message>> = Vec::with_capacity(columns);

    for element in elements {
        current_row_elements.push(element);

        if current_row_elements.len() == columns {
            grid_rows.push(
                row(std::mem::replace(
                    &mut current_row_elements,
                    Vec::with_capacity(columns),
                ))
                .spacing(10)
                .into(),
            );
        }
    }

    if !current_row_elements.is_empty() {
        grid_rows.push(row(current_row_elements).spacing(10).into());
    }

    column(grid_rows).spacing(10).into()
}

pub fn sort_tiles_by_type(tile: &Hai) -> usize {
    crate::implements::tiles::tile_to_index(tile)
}

pub fn insert_tile_sorted(tiles: &mut Vec<Hai>, tile: Hai) {
    let key = sort_tiles_by_type(&tile);
    let pos = tiles
        .binary_search_by_key(&key, sort_tiles_by_type)
        .unwrap_or_else(|pos| pos);
    tiles.insert(pos, tile);
}

pub fn tile_button<'a>(
    content: Element<'a, Message>,
    msg: Message,
    style: ColoredButtonStyle,
) -> Element<'a, Message> {
    button(content)
        .style(theme::Button::Custom(Box::new(style)))
        .on_press(msg)
        .padding(5)
        .into()
}

pub fn action_button<'a>(
    label: &str,
    msg: Message,
    style: ColoredButtonStyle,
) -> Element<'a, Message> {
    button(text(label))
        .style(theme::Button::Custom(Box::new(style)))
        .on_press(msg)
        .into()
}

pub fn cancel_button<'a>() -> Element<'a, Message> {
    action_button(
        "Cancel",
        Message::CancelSelection,
        ColoredButtonStyle::DANGER,
    )
}

pub fn tile_image_button<'a>(
    handle: iced::widget::image::Handle,
    width: u16,
    msg: Message,
    style: theme::Button,
) -> Element<'a, Message> {
    button(iced::widget::Image::new(handle).width(width))
        .on_press(msg)
        .style(style)
        .padding(5)
        .into()
}

pub fn tile_image<'a>(handle: iced::widget::image::Handle, width: u16) -> Element<'a, Message> {
    iced::widget::Image::new(handle).width(width).into()
}
