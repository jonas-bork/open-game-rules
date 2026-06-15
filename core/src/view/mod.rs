use serde::{Deserialize, Serialize};

use crate::{
    game_rules::GameRule,
    model::{Model, game_details, game_overview},
};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ViewModel {
    #[default]
    Uninitialized,
    GamesOverview(GamesOverviewViewModel),
    GameDetails(GameDetailsViewModel),
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct GamesOverviewViewModel {
    pub game_rules: Vec<(String, GameRule)>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct GameDetailsViewModel {
    pub game: GameRule,
}

impl From<&Model> for ViewModel {
    fn from(model: &Model) -> Self {
        match model {
            Model::Uninitialized => Self::Uninitialized,
            Model::GameOverview(model) => Self::GamesOverview(model.into()),
            Model::GameDetails(model) => Self::GameDetails(model.into()),
        }
    }
}

impl From<&game_overview::Model> for GamesOverviewViewModel {
    fn from(model: &game_overview::Model) -> Self {
        Self {
            game_rules: model
                .game_rules
                .iter()
                .map(|(id, rule)| (id.clone(), rule.clone()))
                .collect(),
        }
    }
}

impl From<&game_details::Model> for GameDetailsViewModel {
    fn from(model: &game_details::Model) -> Self {
        Self {
            game: model.game.clone(),
        }
    }
}
