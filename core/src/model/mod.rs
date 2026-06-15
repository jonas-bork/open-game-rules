pub mod game_details;
pub mod game_overview;
pub mod outcome;

use crate::Effect;
use crux_core::Command;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub enum Model {
    #[default]
    Uninitialized,
    GameOverview(game_overview::Model),
    GameDetails(game_details::Model),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Start,
    GameOverview(game_overview::Event),
    GameDetails(game_details::Event),
}

impl Model {
    pub fn update(&mut self, event: Event) -> Command<Effect, Event> {
        match event {
            Event::Start => {
                let (model, cmd) = game_overview::Model::start().into_parts();
                *self = Self::GameOverview(model);
                cmd
            }
            Event::GameOverview(event) => self.update_game_overview(event),
            Event::GameDetails(event) => self.update_game_details(event),
        }
    }

    fn update_game_overview(&mut self, event: game_overview::Event) -> Command<Effect, Event> {
        let owned = std::mem::take(self);
        let Self::GameOverview(model) = owned else {
            *self = owned;
            return Command::done();
        };

        let (status, command) = model
            .update(event)
            .map_event(Event::GameOverview)
            .into_parts();

        match status {
            outcome::Status::Continue(model) => {
                *self = Self::GameOverview(model);
                command
            }
            outcome::Status::Complete(transition) => match transition {
                game_overview::Transition::GameDetails(game) => {
                    let (model, start_command) = game_details::Model::start(game).into_parts();
                    *self = Self::GameDetails(model);
                    command.and(start_command)
                }
            },
        }
    }

    fn update_game_details(&mut self, event: game_details::Event) -> Command<Effect, Event> {
        let owned = std::mem::take(self);
        let Self::GameDetails(model) = owned else {
            *self = owned;
            return Command::done();
        };

        let (status, command) = model
            .update(event)
            .map_event(Event::GameDetails)
            .into_parts();

        match status {
            outcome::Status::Continue(model) => {
                *self = Self::GameDetails(model);
                command
            }
            outcome::Status::Complete(transition) => match transition {
                game_details::Transition::GamesOverview => {
                    let (model, start_command) = game_overview::Model::start().into_parts();
                    *self = Self::GameOverview(model);
                    command.and(start_command)
                }
            },
        }
    }
}
