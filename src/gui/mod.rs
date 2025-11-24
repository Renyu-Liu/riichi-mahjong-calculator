pub mod components;
pub mod messages;
pub mod state;
pub mod styles;
pub mod update;
pub mod view;

use self::messages::Message;
use self::state::RiichiGui;
use self::update::Update;
use self::view::View;
use iced::{Element, Sandbox, Settings};

pub fn run() -> iced::Result {
    RiichiGui::run(Settings::default())
}

impl Sandbox for RiichiGui {
    type Message = Message;

    fn new() -> Self {
        Self::new()
    }

    fn title(&self) -> String {
        String::from("Riichi Mahjong Calculator")
    }

    fn update(&mut self, message: Message) {
        Update::update(self, message)
    }

    fn view(&self) -> Element<'_, Message> {
        View::view(self)
    }
}
