use crux_core::{Command, render::render};
use open_game_rules_data_builder::Game;
use serde::{Deserialize, Serialize};

use crate::{
    Effect,
    model::{NavigateEvent, outcome::Started},
};

pub struct Model;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    SelectGame(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Transition {
    GameDetails(Game),
}

impl Model {
    pub fn start() -> Started<Self, crate::Event> {
        Started {
            state: Self,
            command: render(),
        }
    }

    pub fn update(&mut self, event: Event) -> Command<Effect, crate::Event> {
        match event {
            Event::SelectGame(game_id) => {
                Command::event(crate::Event::Navigate(NavigateEvent::GameDetails {
                    game_id,
                }))
            }
        }
    }
}
