mod components;
mod core;

use std::rc::Rc;

use leptos::prelude::*;
use leptos_router::{
    NavigateOptions,
    components::Router,
    hooks::{use_location, use_navigate},
};
use open_game_rules_core::{
    Event, NavigateEvent,
    view::{PageView, ViewModel},
};

use crate::components::{
    DispatchContext, game_details::GameDetailsView, games_overview::GamesOverviewView,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <AppContent />
        </Router>
    }
}
#[component]
fn AppContent() -> impl IntoView {
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

    let location = use_location();
    let navigate = use_navigate();

    Effect::new({
        let dispatch = dispatch.clone();
        move |_| {
            let browser_path = location.pathname.get();

            let core_path = view.with_untracked(|v| match v {
                ViewModel::Initialized(app) => app.path.clone(),
                ViewModel::Uninitialized => String::new(),
            });

            if browser_path != core_path {
                dispatch.run(Event::Navigate(NavigateEvent::Path(browser_path)));
            }
        }
    });

    Effect::new(move |_| {
        let core_path = view.with(|v| match v {
            ViewModel::Initialized(app) => app.path.clone(),
            ViewModel::Uninitialized => String::new(),
        });
        let browser_path = location.pathname.get_untracked();

        if !core_path.is_empty() && browser_path != core_path {
            navigate(&core_path, NavigateOptions::default());
        }
    });

    let game_overview_vm = Memo::new(move |_| {
        view.with(|v| match v {
            ViewModel::Initialized(app) => match &app.page {
                PageView::GamesOverview(vm) => vm.clone(),
                _ => open_game_rules_core::view::GamesOverviewViewModel::default(),
            },
            _ => open_game_rules_core::view::GamesOverviewViewModel::default(),
        })
    });

    let game_details_vm = Memo::new(move |_| {
        view.with(|v| match v {
            ViewModel::Initialized(app) => match &app.page {
                PageView::GameDetails(vm) => vm.clone(),
                _ => open_game_rules_core::view::GameDetailsViewModel::default(),
            },
            _ => open_game_rules_core::view::GameDetailsViewModel::default(),
        })
    });

    view! {
        <div class="max-w-xl mx-auto px-4 py-8">
            {move || {
                view.with(|v| match v {
                    ViewModel::Uninitialized => {
                        view! { <p>"Loading..."</p> }.into_any()
                    }
                    ViewModel::Initialized(app) => match &app.page {
                        PageView::GamesOverview(_) => {
                            view! { <GamesOverviewView vm=game_overview_vm/> }.into_any()
                        }
                        PageView::GameDetails(_) => {
                            view! { <GameDetailsView vm=game_details_vm/> }.into_any()
                        }
                    }
                })
            }}
        </div>
    }
}
