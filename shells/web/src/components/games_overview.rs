use leptos::prelude::*;
use open_game_rules_core::view::GamesOverviewViewModel;

use crate::{
    components::{common::button::Button, game_badges::GameBadges, use_dispatch},
    core::Event,
};

use open_game_rules_core::GamesOverviewEvent;

#[component]
pub fn games_overview_view(#[prop(into)] vm: Signal<GamesOverviewViewModel>) -> impl IntoView {
    let dispatch = use_dispatch();

    view! {
        <div class="py-8 px-4 text-left md:py-12">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Open Game Rules</h2>

            {move || {
                vm.read().clone().game_rules.into_iter().map(|(id, rule)| {
                    view! {
                        <div class="mb-4 rounded-xl border border-gray-200 bg-white shadow-sm transition-shadow hover:shadow-md">
                            <div class="p-5 flex flex-col items-start gap-2">
                                <p class="text-lg font-semibold text-gray-800">{rule.name}</p>

                                <GameBadges equipment={rule.equipment} players={rule.players} tags={rule.tags} complexity={rule.complexity} playing_time={rule.playing_time} />

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
