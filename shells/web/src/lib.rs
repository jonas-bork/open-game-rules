mod components;
mod core;

use std::rc::Rc;

use leptos::prelude::*;
use open_game_rules_core::{Event, view::ViewModel};

use crate::components::{
    DispatchContext, game_details::GameDetailsView, games_overview::GamesOverviewView,
};

#[component]
pub fn App() -> impl IntoView {
    let core = core::new();
    let (view, set_view) = signal(core.view());

    let dispatch_core = Rc::clone(&core);
    let dispatch = UnsyncCallback::new(move |event: Event| {
        core::update(&dispatch_core, event, set_view);
    });
    provide_context(DispatchContext(dispatch));

    Effect::new(move |_| {
        core::update(&core, Event::Start, set_view);
    });

    let game_overview_vm = Memo::new(move |_| {
        view.with(|v| match v {
            ViewModel::GamesOverview(games_overview_view_model) => {
                games_overview_view_model.clone()
            }
            _ => open_game_rules_core::view::GamesOverviewViewModel::default(),
        })
    });

    let game_details_vm = Memo::new(move |_| {
        view.with(|v| match v {
            ViewModel::GameDetails(model) => model.clone(),
            _ => open_game_rules_core::view::GameDetailsViewModel::default(),
        })
    });

    view! {
        <div class="max-w-xl mx-auto px-4 py-8">
            // <ScreenHeader
            //     title="Crux Weather"
            //     subtitle="Rust Core, Rust Shell (Leptos)"
            //     icon=phosphor_leptos::CROWN
            // />
            <Show when=move || view.with(|v| matches!(v, ViewModel::Uninitialized))>
                <p>Loading</p>
            </Show>
            <Show when=move || view.with(|v| matches!(v, ViewModel::GamesOverview(_)))>
                <GamesOverviewView vm=game_overview_vm/>
            </Show>
            <Show when=move || view.with(|v| matches!(v, ViewModel::GameDetails(_)))>
                <GameDetailsView vm=game_details_vm/>
            </Show>
        </div>
    }
}
