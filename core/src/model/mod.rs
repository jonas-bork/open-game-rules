use crux_core::render::render;
use serde::{Deserialize, Serialize};

use crate::{
    Effect,
    game_rules::{self, GameRules},
};

#[derive(Default)]
pub struct Model {
    pub game_rules: GameRules,
    pub count: isize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Start,
    Increment,
    Decrement,
    Reset,
}

impl Model {
    pub fn update(&mut self, event: Event) -> crux_core::Command<Effect, Event> {
        match event {
            Event::Increment => self.count += 1,
            Event::Decrement => self.count -= 1,
            Event::Reset => self.count = 0,
            Event::Start => {
                self.game_rules =
                    game_rules::load_all_rules().expect("failed to deserialize the generated games")
            }
        }

        render()
    }
}
