mod range;
pub mod read;

use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::bail;
use serde::Deserialize;
use serde::Serialize;

use crate::range::Range;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Equipment {
    #[default]
    Cards,
    Dice,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum Complexity {
    #[default]
    Light,
    MediumLight,
    Medium,
    MediumHeavy,
    Heavy,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct GameMetadata {
    pub id: String,
    pub name: String,
    pub equipment: Vec<Equipment>,
    pub complexity: Complexity,
    pub tags: Vec<String>,
    pub players: Players,
    pub playing_time: PlayingTime,
}

pub type Minutes = u8;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PlayingTime {
    Exact(Minutes),
    Range(Range<Minutes>),
}

pub type PlayerCount = u8;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Players {
    Exact(PlayerCount),
    Range { min: PlayerCount, max: PlayerCount },
}

impl TryFrom<&str> for Players {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();

        if let Some((min_str, max_str)) = value.split_once('-') {
            let min = min_str
                .trim()
                .parse::<PlayerCount>()
                .context("Failed to parse the minimum player count")?;

            let max = max_str
                .trim()
                .parse::<PlayerCount>()
                .context("Failed to parse the maximum player count")?;

            if min > max {
                bail!("Minimum player count cannot be greater than maximum");
            }

            Ok(Players::Range { min, max })
        } else {
            let count = value
                .parse::<PlayerCount>()
                .context("Failed to parse the exact player count")?;

            Ok(Players::Exact(count))
        }
    }
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

impl Default for PlayingTime {
    fn default() -> Self {
        Self::Exact(0)
    }
}
