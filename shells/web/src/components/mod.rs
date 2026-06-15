use leptos::prelude::{UnsyncCallback, expect_context};

use crate::core::Event;

pub mod common;
pub mod game_details;
pub mod games_overview;

pub type SendEvent = UnsyncCallback<Event>;

#[derive(Clone)]
pub struct DispatchContext(pub SendEvent);

#[must_use]
pub fn use_dispatch() -> SendEvent {
    expect_context::<DispatchContext>().0
}
