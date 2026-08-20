use leptos::prelude::*;
use open_game_rules_core::view::GamesOverviewViewModel;
use phosphor_leptos::Icon;

use crate::{
    components::{game_badges::GameBadges, use_dispatch},
    core::Event,
};

use open_game_rules_core::GamesOverviewEvent;

#[component]
pub fn games_overview_view(#[prop(into)] vm: Signal<GamesOverviewViewModel>) -> impl IntoView {
    let dispatch = use_dispatch();

    view! {
        <div class="py-8 px-4 text-left md:py-12">
            <h2 class="text-2xl font-bold mb-4">Open Game Rules</h2>

            {move || {
                vm.read().clone().game_rules.into_iter().map(|(id, rule)| {
                    view! {
                        <div class="mb-4 rounded-xl border border-outline bg-surface-container text-on-surface-container light:shadow-sm transition-shadow light:hover:shadow-md"
                            on:click=move |_| {
                                dispatch.run(Event::GameOverview(GamesOverviewEvent::SelectGame(id.clone())));
                            }
                        >
                            <div class="p-5 flex flex-col items-start gap-2 cursor-pointer">
                                <div class="flex flex-row items-center text-lg font-semibold">
                                    <span>{rule.name}</span>
                                    <Icon icon=phosphor_leptos::CARET_RIGHT size="20px" />
                                </div>

                                <GameBadges equipment={rule.equipment} players={rule.players} tags={rule.tags} complexity={rule.complexity} playing_time={rule.playing_time} />
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}
