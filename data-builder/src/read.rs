use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::{
    Complexity, Equipment, Game, GameMetadata, Minutes, Players, PlayingTime, range::Range,
};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataContent {
    pub name: String,
    pub equipment: Vec<Equipment>,
    pub complexity: u8,
    pub tags: Vec<String>,
    pub players: String,
    pub playing_time: String,
}

impl GameMetadata {
    fn try_from_content(id: String, content: MetadataContent) -> Result<Self> {
        Ok(Self {
            id: id,
            name: content.name,
            equipment: content.equipment,
            complexity: match content.complexity {
                1 => Complexity::Light,
                2 => Complexity::MediumLight,
                3 => Complexity::Medium,
                4 => Complexity::MediumHeavy,
                5 => Complexity::Heavy,
                _ => bail!("Unsupported complexity level '{}'", content.complexity),
            },
            tags: content.tags,
            players: Players::try_from(content.players.as_str())
                .context("failed to parse players")?,
            playing_time: PlayingTime::try_from(content.playing_time.as_str())
                .context("failed to parse playing time")?,
        })
    }
}

impl TryFrom<&str> for PlayingTime {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        let s = s.trim();

        if let Some((min_str, max_str)) = s.split_once('-') {
            let min = min_str
                .trim()
                .parse::<Minutes>()
                .context("Failed to parse the minimum playing time")?;

            let max = max_str
                .trim()
                .parse::<Minutes>()
                .context("Failed to parse the maximum playing time")?;

            let range = Range::new(min, max)
                .context("Minimum playing time cannot be greater than maximum")?;

            Ok(PlayingTime::Range(range))
        } else {
            let count = s
                .parse::<Minutes>()
                .context("Failed to parse the exact playing time")?;

            Ok(PlayingTime::Exact(count))
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
                    let metadata =
                        GameMetadata::try_from_content(game_id.clone(), metadata_content).unwrap();

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
