use crux_core::{Command, render::render};
use open_game_rules_data_builder::Game;
use serde::{Deserialize, Serialize};

use crate::{
    Effect,
    model::{NavigateEvent, outcome::Started},
};

pub struct Model {
    pub game: Game,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    GoBack,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Transition {
    GamesOverview,
}

impl Model {
    pub fn start(game: Game) -> Started<Self, crate::Event> {
        Started {
            state: Self { game },
            command: render(),
        }
    }

    pub fn update(&mut self, event: Event) -> Command<Effect, crate::Event> {
        match event {
            Event::GoBack => Command::event(crate::Event::Navigate(NavigateEvent::GamesOverview)),
        }
    }
}
