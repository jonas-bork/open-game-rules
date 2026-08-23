use std::env;
use std::path::Path;

const DATA_DIR: &str = "../data/games";
const OUTPUT_FILE_NAME: &str = "generated-games-database.json";

fn main() {
    // Tell Cargo to re-run this script ONLY if the /data/games folder changes.
    println!("cargo:rerun-if-changed={DATA_DIR}");

    let data_dir = Path::new(DATA_DIR);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join(OUTPUT_FILE_NAME);

    open_game_rules_data_builder::build_games(data_dir, &out_file);
}
