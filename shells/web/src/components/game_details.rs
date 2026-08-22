use leptos::prelude::*;
use open_game_rules_core::view::GameDetailsViewModel;
use phosphor_leptos::Icon;

use crate::{
    components::{
        common::icon_button::{self, IconButton},
        game_badges::GameBadges,
        markdown_renderer::MarkdownRenderer,
        use_dispatch,
    },
    core::Event,
};

use open_game_rules_core::GameDetailsEvent;

#[component]
pub fn game_details_view(#[prop(into)] vm: Signal<GameDetailsViewModel>) -> impl IntoView {
    let dispatch = use_dispatch();

    move || {
        let rule = vm.read().game.clone();
        let container_transition_name_style =
            format!("view-transition-name: game-container-{}", rule.id);
        let title_transition_name_style = format!("view-transition-name: game-title-{}", rule.id);
        let go_back = move || dispatch.run(Event::GameDetails(GameDetailsEvent::GoBack));
        view! {
            <div class="py-8 px-4 text-left md:py-12 flex flex-col gap-4 items-start" style=container_transition_name_style>
                <div class="flex flex-row items-center gap-1 text-2xl font-bold">
                    <IconButton color=icon_button::Color::Standard on_click={move |_| go_back()}>
                        <Icon icon=phosphor_leptos::CARET_LEFT size="24px" />
                    </IconButton>
                    <h1 style=title_transition_name_style>{rule.name}</h1>
                </div>
                <GameBadges game_id=rule.id equipment={rule.equipment} players={rule.players} tags={rule.tags} complexity={rule.complexity} playing_time={rule.playing_time} />
                <div class="prose">
                    <MarkdownRenderer markdown={rule.rules} />
                </div>
            </div>
        }
    }
}
