use leptos::prelude::*;
use open_game_rules_core::view::GamesOverviewViewModel;
use phosphor_leptos::Icon;

use crate::{
    components::{
        common::{
            badge::{Badge, BadgeVariant},
            button::Button,
        },
        use_dispatch,
    },
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

                                // Badges
                                <div class="flex flex-col gap-x-2 gap-y-1">
                                    <div class="flex flex-row gap-x-2 gap-y-1">
                                        // Equipment
                                        <Badge variant=BadgeVariant::Blue>
                                            <Icon icon=phosphor_leptos::PACKAGE size="18px" />
                                            <span>{rule.equipment.into_iter().next().unwrap()}</span>
                                        </Badge>

                                        // Players
                                        <Badge variant=BadgeVariant::Blue>
                                            <Icon icon=phosphor_leptos::USERS size="18px" />
                                            <span>
                                                {match rule.players {
                                                    open_game_rules_core::Players::Exact(n) => n.to_string(),
                                                    open_game_rules_core::Players::Range { min, max } => format!("{min} - {max}"),
                                                }}
                                            </span>
                                        </Badge>

                                        // TODO: Add difficulty (also to the core)
                                    </div>
                                    <div class="flex flex-row gap-x-2 gap-y-1">
                                        {
                                            rule.tags.into_iter().map(|tag| {
                                                view! {
                                                    <Badge>{tag.clone()}</Badge>
                                                }
                                            }).collect::<Vec<_>>()
                                        }
                                    </div>
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
