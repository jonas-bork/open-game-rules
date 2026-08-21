use leptos::prelude::*;
use open_game_rules_core::view::GamesOverviewViewModel;
use phosphor_leptos::Icon;

use crate::{
    components::{
        common::card::{Card, CardVariant},
        game_badges::GameBadges,
        use_dispatch,
    },
    core::Event,
};

use open_game_rules_core::GamesOverviewEvent;

#[component]
pub fn games_overview_view(#[prop(into)] vm: Signal<GamesOverviewViewModel>) -> impl IntoView {
    let dispatch = use_dispatch();

    view! {
        <div class="py-8 px-4 text-left flex flex-col gap-4">
            <h2 class="text-2xl font-bold">Open Game Rules</h2>

            {move || {
                vm.read().clone().game_rules.into_iter().map(|(id, rule)| {
                    let transition_name_style = format!("view-transition-name: game-title-{}", id);
                    view! {
                        <Card
                            variant=CardVariant::Outlined
                            on_click=move |_| {
                                dispatch.run(Event::GameOverview(GamesOverviewEvent::SelectGame(id.clone())));
                            }
                        >
                            <div class="flex flex-col items-start gap-2 cursor-pointer w-full">
                                <div class="flex justify-between items-center text-lg font-semibold w-full">
                                    <span style=transition_name_style>{rule.name}</span>
                                    <Icon icon=phosphor_leptos::CARET_RIGHT size="20px" />
                                </div>

                                <GameBadges game_id=rule.id equipment={rule.equipment} players={rule.players} tags={rule.tags} complexity={rule.complexity} playing_time={rule.playing_time} />
                            </div>
                        </Card>
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}
