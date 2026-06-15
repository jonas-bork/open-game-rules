use leptos::prelude::*;
use open_game_rules_core::view::GamesOverviewViewModel;

use crate::{
    components::{common::button::Button, use_dispatch},
    core::Event,
};

use open_game_rules_core::GamesOverviewEvent;

#[component]
pub fn games_overview_view(#[prop(into)] vm: Signal<GamesOverviewViewModel>) -> impl IntoView {
    let dispatch = use_dispatch();

    view! {
        <div class="game-rules-list section text-left">
            <h2 class="title is-4">Games</h2>

            {move || {
                vm.read().clone().game_rules.into_iter().map(|(id, rule)| {
                    view! {
                        <div class="card mb-3">
                            <div class="card-content">
                                <p class="title is-5">{rule.metadata.name}</p>
                                // TODO: Add equipment, tags, players and also difficulty (also add the last one to the core)

                                <Button
                                    label="Details"
                                    on_click=UnsyncCallback::new(move |()| {
                                        dispatch.run(Event::GameOverview(GamesOverviewEvent::SelectGame(id.clone())));
                                    })
                                />
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}
