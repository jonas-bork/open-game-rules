use open_game_rules_data_builder::Game;
use std::collections::HashMap;

pub type Games = HashMap<String, Game>;

const GENERATED_GAMES_JSON: &str =
    include_str!(concat!(env!("OUT_DIR"), "/generated-games-database.json"));

pub fn load_all_rules() -> Result<Games, serde_json::Error> {
    serde_json::from_str(GENERATED_GAMES_JSON)
}
