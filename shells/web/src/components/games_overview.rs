use leptos::prelude::*;
use open_game_rules_core::{Equipment, view::GamesOverviewViewModel};

use crate::{
    components::{common::button::Button, use_dispatch},
    core::Event,
};

use open_game_rules_core::GamesOverviewEvent;

#[component]
pub fn games_overview_view(#[prop(into)] vm: Signal<GamesOverviewViewModel>) -> impl IntoView {
    let dispatch = use_dispatch();

    view! {
        <div class="py-8 px-4 text-left md:py-12">
            <h2 class="mb-6 text-2xl font-bold text-gray-900">Games</h2>

            {move || {
                vm.read().clone().game_rules.into_iter().map(|(id, rule)| {
                    view! {
                        <div class="mb-4 rounded-xl border border-gray-200 bg-white shadow-sm transition-shadow hover:shadow-md">
                            <div class="p-5 flex flex-col items-start">
                                <p class="mb-4 text-lg font-semibold text-gray-800">{rule.name}</p>
                                <div class="flex flex-row">
                                    // TODO: Add equipment, players and also difficulty (also add the last one to the core)
                                    <div class="rounded-md bg-rose-500">
                                        <span>{rule.equipment.first().unwrap().clone()}</span>
                                    </div>
                                </div>
                                <div class="flex flex-row">
                                    // TODO: add tags
                                </div>

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
