pub mod read;

use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Equipment {
    #[default]
    Cards,
    Dice,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct GameMetadata {
    pub name: String,
    pub equipment: Vec<Equipment>,
    pub tags: Vec<String>,
    pub players: Players,
}

pub type PlayerCount = u8;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Players {
    Exact(PlayerCount),
    Range { min: PlayerCount, max: PlayerCount },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Game {
    pub metadata: GameMetadata,
    pub rules: String,
}

pub fn build_games(data_dir: &Path, out_file: &Path) {
    let games = read::read(data_dir);
    let json_output = serde_json::to_string(&games).expect("Failed to serialize to JSON");
    fs::write(&out_file, json_output).expect("Failed to write generated_database.json");

    // DEBUGGING
    // let json_output =
    //     serde_json::to_string_pretty(&all_games).expect("Failed to serialize to JSON");
    // fs::write("debug-games-database.json", &json_output).unwrap();
}

impl Default for Players {
    fn default() -> Self {
        Self::Exact(0)
    }
}
