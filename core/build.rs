use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct GameMetadata {
    pub name: String,
    pub equipment: Vec<String>,
    pub tags: Vec<String>,
    pub players: Players,
}

#[derive(Serialize, Deserialize, Debug)]
struct Players {
    pub min: Option<u8>,
    pub max: Option<u8>,
}

#[derive(Serialize, Debug)]
struct GameRule {
    pub metadata: GameMetadata,
    pub rules: String,
}

const DATA_DIR: &str = "../data/games";
const OUTPUT_FILE_NAME: &str = "generated-games-database.json";

fn main() {
    // Tell Cargo to re-run this script ONLY if the /data/games folder changes.
    println!("cargo:rerun-if-changed={DATA_DIR}");

    let data_dir = Path::new(DATA_DIR);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(OUTPUT_FILE_NAME);

    let mut all_games: HashMap<String, GameRule> = HashMap::new();

    if let Ok(entries) = fs::read_dir(data_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                // The folder name is the ID
                let game_id = path.file_name().unwrap().to_string_lossy().to_string();

                let metadata_path = path.join("metadata.yml");
                let rules_path = path.join("rules.md");

                if metadata_path.exists() && rules_path.exists() {
                    let yaml_content = fs::read_to_string(&metadata_path)
                        .unwrap_or_else(|_| panic!("Failed to read {:?}", metadata_path));

                    let metadata: GameMetadata = serde_yaml::from_str(&yaml_content)
                        .unwrap_or_else(|_| panic!("Invalid YAML in {:?}", metadata_path));

                    let rules = fs::read_to_string(&rules_path)
                        .unwrap_or_else(|_| panic!("Failed to read {:?}", rules_path));

                    let existing_game =
                        all_games.insert(game_id.clone(), GameRule { metadata, rules });
                    if existing_game.is_some() {
                        panic!("two games with ID '{game_id}' exist");
                    }
                } else {
                    panic!("game '{game_id}' is missing metadata.yml or rules.md");
                }
            }
        }
    }

    let json_output = serde_json::to_string(&all_games).expect("Failed to serialize to JSON");
    fs::write(&dest_path, json_output).expect("Failed to write generated_database.json");

    // DEBUGGING
    // let json_output =
    //     serde_json::to_string_pretty(&all_games).expect("Failed to serialize to JSON");
    // fs::write("debug-games-database.json", &json_output).unwrap();
}
