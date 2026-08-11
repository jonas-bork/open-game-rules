use leptos::prelude::*;
use open_game_rules_core::view::GameDetailsViewModel;

use crate::{
    components::{
        common::button::Button, game_badges::GameBadges, markdown_renderer::MarkdownRenderer,
        use_dispatch,
    },
    core::Event,
};

use open_game_rules_core::GameDetailsEvent;

#[component]
pub fn game_details_view(#[prop(into)] vm: Signal<GameDetailsViewModel>) -> impl IntoView {
    let dispatch = use_dispatch();

    view! {
        <div class="py-8 px-4 text-left md:py-12 flex flex-col gap-4 items-start">
            {move || {
                let rule = vm.read().game.clone();
                view! {
                    <h1 class="text-2xl font-bold text-gray-900">{rule.name}</h1>
                    <GameBadges equipment={rule.equipment} players={rule.players} tags={rule.tags} complexity={rule.complexity} playing_time={rule.playing_time} />
                    <div class="prose">
                        <MarkdownRenderer markdown={rule.rules} />
                    </div>

                    <Button
                        label="Back"
                        on_click=UnsyncCallback::new(move |()| {
                            dispatch.run(Event::GameDetails(GameDetailsEvent::GoBack));
                        })
                    />
                }
            }}
        </div>
    }
}
