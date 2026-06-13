use serde::{Deserialize, Serialize};

use crate::{game_rules::GameRule, model::Model};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {
    pub game_rules: Vec<GameRule>,
}

impl From<&Model> for ViewModel {
    fn from(model: &Model) -> Self {
        ViewModel {
            game_rules: model.game_rules.values().cloned().collect(),
        }
    }
}
