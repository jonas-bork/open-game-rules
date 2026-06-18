use crux_core::{Command, render::render};
use open_game_rules_data_builder::Game;
use serde::{Deserialize, Serialize};

use crate::model::outcome::{Outcome, Started};

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

    pub fn update(self, event: Event) -> Outcome<Self, Transition, Event> {
        match event {
            Event::GoBack => Outcome::complete(Transition::GamesOverview, Command::done()),
        }
    }
}
