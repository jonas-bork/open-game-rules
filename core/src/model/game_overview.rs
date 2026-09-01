use crux_core::{Command, render::render};
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

impl Model {
    pub fn start() -> Started<Self, crate::Event> {
        Started {
            state: Self,
            command: render(),
        }
    }

    #[allow(
        clippy::needless_pass_by_ref_mut,
        clippy::unused_self,
        clippy::needless_pass_by_value
    )]
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
