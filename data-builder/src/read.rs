use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;

use crate::{Complexity, Equipment, Game, GameMetadata, Players};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataContent {
    pub name: String,
    pub equipment: Vec<Equipment>,
    pub complexity: u8,
    pub tags: Vec<String>,
    pub players: String,
}

impl From<MetadataContent> for GameMetadata {
    fn from(content: MetadataContent) -> Self {
        Self {
            name: content.name,
            equipment: content.equipment,
            complexity: match content.complexity {
                1 => Complexity::Light,
                2 => Complexity::MediumLight,
                3 => Complexity::Medium,
                4 => Complexity::MediumHeavy,
                5 => Complexity::Heavy,
                _ => panic!("Unsupported complexity level '{}'", content.complexity),
            },
            tags: content.tags,
            players: Players::try_from(content.players.as_str()).unwrap(),
        }
    }
}

pub fn read(data_dir: &Path) -> HashMap<String, Game> {
    let mut all_games: HashMap<String, Game> = HashMap::new();
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
                        .unwrap_or_else(|e| panic!("Failed to read {metadata_path:?}: {e:?}"));

                    let metadata_content: MetadataContent = serde_yaml::from_str(&yaml_content)
                        .unwrap_or_else(|e| panic!("Invalid YAML in {metadata_path:?}: {e:?}"));
                    let metadata = GameMetadata::from(metadata_content);

                    let rules = fs::read_to_string(&rules_path)
                        .unwrap_or_else(|e| panic!("Failed to read {rules_path:?}: {e:?}"));

                    let existing_game = all_games.insert(game_id.clone(), Game { metadata, rules });
                    if existing_game.is_some() {
                        panic!("two games with ID '{game_id}' exist");
                    }
                } else {
                    panic!("game '{game_id}' is missing metadata.yml or rules.md");
                }
            }
        }
    }

    all_games
}
