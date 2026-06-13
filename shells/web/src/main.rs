mod core;

use leptos::prelude::*;
use open_game_rules_core::Event;

#[component]
fn RootComponent() -> impl IntoView {
    let core = core::new();
    let (view, render) = signal(core.view());
    let (event, _) = signal(Event::Start);

    Effect::new(move |_| {
        core::update(&core, event.get(), render);
    });

    view! {
        <section class="box container has-text-centered m-5">
            <div class="game-rules-list section text-left">
                <h2 class="title is-4">Games</h2>

                {move || {
                    view.get().game_rules.into_iter().map(|rule| {
                        view! {
                            <div class="card mb-3">
                                <div class="card-content">
                                    <p class="title is-5">{rule.metadata.name}</p>
                                    // TODO: Add equipment, tags, players and also difficulty (also add the last one to the core)
                                    <p class="subtitle is-6">{rule.rules}</p>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>() // Collect the iterator into a Vec so Leptos can render it
                }}
            </div>        </section>
    }
}

fn main() {
    leptos::mount::mount_to_body(|| {
        view! { <RootComponent /> }
    });
}
