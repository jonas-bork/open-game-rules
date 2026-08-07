use open_game_rules_data_builder::{Equipment, Game, Players};
use serde::{Deserialize, Serialize};

use crate::model::{Model, game_details, game_overview};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ViewModel {
    #[default]
    Uninitialized,
    GamesOverview(GamesOverviewViewModel),
    GameDetails(GameDetailsViewModel),
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct GamesOverviewViewModel {
    pub game_rules: Vec<(String, GameView)>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct GameDetailsViewModel {
    pub game: GameView,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct GameView {
    pub name: String,
    pub rules: String,
    pub equipment: Vec<String>,
    pub tags: Vec<String>,
    pub players: Players,
}

impl From<&Game> for GameView {
    fn from(game: &Game) -> Self {
        Self {
            name: game.metadata.name.clone(),
            rules: game.rules.clone(),
            equipment: game
                .metadata
                .equipment
                .iter()
                .map(equipment_to_string)
                .collect(),
            tags: game.metadata.tags.clone(),
            players: game.metadata.players.clone(),
        }
    }
}

fn equipment_to_string(equipment: &Equipment) -> String {
    match equipment {
        open_game_rules_data_builder::Equipment::Cards => "Cards".to_string(),
        open_game_rules_data_builder::Equipment::Dice => "Dice".to_string(),
    }
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
                .map(|(id, rule)| (id.clone(), rule.into()))
                .collect(),
        }
    }
}

impl From<&game_details::Model> for GameDetailsViewModel {
    fn from(model: &game_details::Model) -> Self {
        Self {
            game: (&model.game).into(),
        }
    }
}
