use leptos::prelude::*;
use open_game_rules_core::view::GameDetailsViewModel;

use crate::{
    components::{common::button::Button, markdown_renderer::MarkdownRenderer, use_dispatch},
    core::Event,
};

use open_game_rules_core::GameDetailsEvent;

#[component]
pub fn game_details_view(#[prop(into)] vm: Signal<GameDetailsViewModel>) -> impl IntoView {
    let dispatch = use_dispatch();

    view! {
        <div class="section text-left">
            {move || {
                let rule = vm.read().game.clone();
                view! {
                    <h1 class="title is-5">{rule.metadata.name}</h1>
                    <div class="card mb-3">
                        <div class="card-content">
                            // TODO: Add equipment, tags, players and also difficulty (also add the last one to the core)
                            <MarkdownRenderer markdown={rule.rules} />

                            <Button
                                label="Back"
                                on_click=UnsyncCallback::new(move |()| {
                                    dispatch.run(Event::GameDetails(GameDetailsEvent::GoBack));
                                })
                            />
                        </div>
                    </div>
                }
            }}
        </div>
    }
}
