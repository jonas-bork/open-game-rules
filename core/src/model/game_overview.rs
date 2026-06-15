use crux_core::{Command, render::render};
use serde::{Deserialize, Serialize};

use crate::{
    game_rules::{self, GameRule, GameRules},
    model::outcome::{Outcome, Started},
};

pub struct Model {
    pub game_rules: GameRules,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    SelectGame(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Transition {
    GameDetails(GameRule),
}

impl Model {
    pub fn start() -> Started<Self, crate::Event> {
        let game_rules =
            game_rules::load_all_rules().expect("failed to deserialize the generated games");

        Started {
            state: Self { game_rules },
            command: render(),
        }
    }

    pub fn update(self, event: Event) -> Outcome<Self, Transition, Event> {
        match event {
            Event::SelectGame(game_id) => {
                let rule = self
                    .game_rules
                    .get(&game_id)
                    .expect("game ID does not exist")
                    .clone();

                Outcome::complete(Transition::GameDetails(rule), Command::done())
            }
        }
    }
}
