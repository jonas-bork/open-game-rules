mod app;
mod effects;
mod game_rules;
mod model;
pub mod view;

pub use open_game_rules_data_builder::{Game, GameMetadata};

pub use app::*;
pub use crux_core::Core;
pub use effects::navigation::NavigationOperation;
pub use model::Event;
pub use model::NavigateEvent;
pub use model::game_details::Event as GameDetailsEvent;
pub use model::game_overview::Event as GamesOverviewEvent;
