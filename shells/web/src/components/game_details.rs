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
        <div class="py-8 px-4 text-left md:py-12">
            {move || {
                let rule = vm.read().game.clone();
                view! {
                    <h1 class="mb-6 text-2xl font-bold text-gray-900">{rule.metadata.name}</h1>
                    <div class="mb-4 rounded-xl border border-gray-200 bg-white shadow-sm">
                        <div class="p-5 flex flex-col items-start">
                            // TODO: Add equipment, tags, players and also difficulty (also add the last one to the core)
                            <div class="mb-6 w-full prose">
                                <MarkdownRenderer markdown={rule.rules} />
                            </div>

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
