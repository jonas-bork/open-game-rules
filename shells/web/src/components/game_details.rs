use leptos::prelude::*;
use open_game_rules_core::view::GameDetailsViewModel;

use crate::{
    components::{common::button::Button, use_dispatch},
    core::Event,
};

use open_game_rules_core::GameDetailsEvent;

#[component]
pub fn game_details_view(#[prop(into)] vm: Signal<GameDetailsViewModel>) -> impl IntoView {
    let dispatch = use_dispatch();

    view! {
        <div class="game-rules-list section text-left">
            {move || {
                let rule = vm.read().game.clone();
                view! {
                    <h2 class="title is-4">{}</h2>
                    <div class="card mb-3">
                        <div class="card-content">
                            <p class="title is-5">{rule.metadata.name}</p>
                            // TODO: Add equipment, tags, players and also difficulty (also add the last one to the core)
                            <p class="subtitle is-6">{rule.rules}</p>

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
