use leptos::prelude::*;
use open_game_rules_core::view::GameDetailsViewModel;
use phosphor_leptos::Icon;

use crate::{
    components::{game_badges::GameBadges, markdown_renderer::MarkdownRenderer, use_dispatch},
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
                let go_back = move || {dispatch.run(Event::GameDetails(GameDetailsEvent::GoBack))};
                view! {
                    <div class="flex flex-row items-center text-2xl font-bold">
                        <div class="cursor-pointer" tabindex="0" on:click={move |_| go_back()} on:keydown=move |e| {
                            if e.key() == "Enter" || e.key() == " " {go_back()}
                        }>
                            <Icon icon=phosphor_leptos::CARET_LEFT size="24px" />
                        </div>
                        <h1>{rule.name}</h1>
                    </div>
                    <GameBadges equipment={rule.equipment} players={rule.players} tags={rule.tags} complexity={rule.complexity} playing_time={rule.playing_time} />
                    <div class="prose">
                        <MarkdownRenderer markdown={rule.rules} />
                    </div>
                }
            }}
        </div>
    }
}
