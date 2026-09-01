pub mod game_details;
pub mod game_overview;
pub mod outcome;

use crate::{
    Effect,
    effects::navigation,
    game_rules::{self, Games},
};
use crux_core::Command;
use serde::{Deserialize, Serialize};

#[derive(Default)]
#[allow(clippy::large_enum_variant)]
pub enum Model {
    #[default]
    Uninitialized,
    Initialized(AppModel),
}

pub struct AppModel {
    pub path: String,
    pub game_rules: Games,
    pub page: PageModel,
    pub logs: Vec<LogMessage>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LogMessage {
    pub severity: Severity,
    pub tag: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl LogMessage {
    pub fn new(severity: Severity, tag: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity,
            tag: tag.into(),
            message: message.into(),
        }
    }
}

pub enum PageModel {
    GamesOverview(game_overview::Model),
    GameDetails(game_details::Model),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Start,
    Log(LogMessage),
    Navigate(NavigateEvent),
    GameOverview(game_overview::Event),
    GameDetails(game_details::Event),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum NavigateEvent {
    Path(String),
    GamesOverview,
    GameDetails { game_id: String },
}

impl Model {
    pub fn update(&mut self, event: Event) -> Command<Effect, Event> {
        match event {
            Event::Start => {
                let Ok(game_rules) = game_rules::load_all_rules() else {
                    return Command::event(Event::Log(LogMessage::new(
                        Severity::Info,
                        "game rules",
                        "failed to deserialize the generated games",
                    )));
                };

                let (model, cmd) = game_overview::Model::start().into_parts();

                *self = Self::Initialized(AppModel {
                    path: "/".to_string(),
                    game_rules,
                    page: PageModel::GamesOverview(model),
                    logs: Vec::new(),
                });

                cmd
            }
            Event::GameOverview(event) => self.update_game_overview(event),
            Event::GameDetails(event) => self.update_game_details(event),
            Event::Navigate(event) => self.navigate(event),
            Event::Log(log_message) => self.log(log_message),
        }
    }

    fn navigate(&mut self, event: NavigateEvent) -> Command<Effect, Event> {
        let Self::Initialized(model) = self else {
            return Command::done();
        };

        match event {
            NavigateEvent::Path(path) => {
                if path == model.path {
                    Command::done()
                } else {
                    match path.as_str() {
                        "" | "/" => Command::event(Event::Navigate(NavigateEvent::GamesOverview)),
                        p if p.starts_with("/game/") => {
                            let id = p.trim_start_matches("/game/").to_string();
                            Command::event(Event::Navigate(NavigateEvent::GameDetails {
                                game_id: id,
                            }))
                        }
                        _ => Command::event(Event::Log(LogMessage::new(
                            Severity::Error,
                            "unknown path",
                            format!("path {path} not found"),
                        ))),
                    }
                }
            }
            NavigateEvent::GamesOverview => {
                model.path = "/".to_string();
                let cmd = navigation::push(model.path.clone());
                let (game_overview_model, start_command) =
                    game_overview::Model::start().into_parts();
                model.page = PageModel::GamesOverview(game_overview_model);
                cmd.and(start_command)
            }
            NavigateEvent::GameDetails { game_id } => {
                let game_id = game_id.to_lowercase();
                model.path = format!("/game/{game_id}");
                let cmd = navigation::push(model.path.clone());
                let Some(game) = model.game_rules.get(&game_id).cloned() else {
                    return Command::event(Event::Log(LogMessage::new(
                        Severity::Error,
                        "game not found",
                        format!("game with ID {game_id} not found"),
                    )));
                };

                let (game_details_model, start_command) =
                    game_details::Model::start(game).into_parts();
                model.page = PageModel::GameDetails(game_details_model);
                cmd.and(start_command)
            }
        }
    }

    fn update_game_overview(&mut self, event: game_overview::Event) -> Command<Effect, Event> {
        if let Self::Initialized(app_model) = self
            && let PageModel::GamesOverview(page_model) = &mut app_model.page
        {
            return page_model.update(event);
        }

        Command::done()
    }

    fn update_game_details(&mut self, event: game_details::Event) -> Command<Effect, Event> {
        if let Self::Initialized(app_model) = self
            && let PageModel::GameDetails(page_model) = &mut app_model.page
        {
            return page_model.update(event);
        }

        Command::done()
    }

    fn log(&mut self, log_message: LogMessage) -> Command<Effect, Event> {
        let Self::Initialized(app_model) = self else {
            return Command::done();
        };

        app_model.logs.push(log_message);
        Command::done()
    }
}
