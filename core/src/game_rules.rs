use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameMetadata {
    pub name: String,
    pub equipment: Vec<String>,
    pub tags: Vec<String>,
    pub players: Players,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Players {
    pub min: Option<u8>,
    pub max: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameRule {
    pub metadata: GameMetadata,
    pub rules: String,
}

pub type GameRules = HashMap<String, GameRule>;

const GENERATED_GAMES_JSON: &str =
    include_str!(concat!(env!("OUT_DIR"), "/generated-games-database.json"));

pub fn load_all_rules() -> Result<GameRules, serde_json::Error> {
    serde_json::from_str(GENERATED_GAMES_JSON)
}
