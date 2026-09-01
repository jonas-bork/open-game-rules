use open_game_rules_data_builder::{Complexity, Equipment, Game, Players, PlayingTime};
use serde::{Deserialize, Serialize};

use crate::{
    game_rules::Games,
    model::{AppModel, Model, PageModel, game_details},
};

#[allow(clippy::large_enum_variant)]
pub enum ViewModel {
    Uninitialized,
    Initialized(AppView),
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct AppView {
    pub path: String,
    pub page: PageView,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum PageView {
    GamesOverview(GamesOverviewViewModel),
    GameDetails(GameDetailsViewModel),
}

impl Default for PageView {
    fn default() -> Self {
        Self::GamesOverview(GamesOverviewViewModel::default())
    }
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
    pub id: String,
    pub name: String,
    pub rules: String,
    pub equipment: String,
    pub tags: Vec<String>,
    pub complexity: String,
    pub players: String,
    pub playing_time: String,
}

const EN_DASH: &str = "–";

impl From<&Game> for GameView {
    fn from(game: &Game) -> Self {
        Self {
            id: game.metadata.id.clone(),
            name: game.metadata.name.clone(),
            rules: game.rules.clone(),
            equipment: equipment_to_string(&game.metadata.equipment),
            tags: game.metadata.tags.clone(),
            complexity: match game.metadata.complexity {
                Complexity::Light => "Light".to_string(),
                Complexity::MediumLight => "Medium light".to_string(),
                Complexity::Medium => "Medium".to_string(),
                Complexity::MediumHeavy => "Medium heavy".to_string(),
                Complexity::Heavy => "Heavy".to_string(),
            },
            players: match game.metadata.players {
                Players::Exact(n) => n.to_string(),
                Players::Range { min, max } => format!("{min}{EN_DASH}{max}"),
            },
            playing_time: match &game.metadata.playing_time {
                PlayingTime::Exact(n) => format!("{n} min"),
                PlayingTime::Range(range) => format!("{}{EN_DASH}{} min", range.from(), range.to()),
            },
        }
    }
}

fn equipment_to_string(equipment: &[Equipment]) -> String {
    let equipment = equipment
        .iter()
        .map(|eq| match eq {
            open_game_rules_data_builder::Equipment::Cards => "Cards",
            open_game_rules_data_builder::Equipment::Dice => "Dice",
        })
        .collect::<Vec<_>>();

    match equipment.as_slice() {
        [] => "none".to_string(),
        [single] => single.to_string(),
        [first, second] => format!("{first} and {second}"),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

impl From<&Model> for ViewModel {
    fn from(model: &Model) -> Self {
        match model {
            Model::Uninitialized => Self::Uninitialized,
            Model::Initialized(app_model) => Self::Initialized(app_model.into()),
        }
    }
}

impl From<&AppModel> for AppView {
    fn from(model: &AppModel) -> Self {
        Self {
            path: model.path.clone(),
            page: match &model.page {
                PageModel::GamesOverview(_) => {
                    PageView::GamesOverview(GamesOverviewViewModel::new(&model.game_rules))
                }
                PageModel::GameDetails(model) => PageView::GameDetails(model.into()),
            },
        }
    }
}

impl GamesOverviewViewModel {
    fn new(game_rules: &Games) -> Self {
        let mut game_rules: Vec<_> = game_rules
            .iter()
            .map(|(id, rule)| (id.clone(), rule.into()))
            .collect();

        game_rules.sort_unstable_by(|game1, game2| game1.0.cmp(&game2.0));

        Self { game_rules }
    }
}

impl From<&game_details::Model> for GameDetailsViewModel {
    fn from(model: &game_details::Model) -> Self {
        Self {
            game: (&model.game).into(),
        }
    }
}
