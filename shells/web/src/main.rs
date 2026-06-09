mod core;

use leptos::prelude::*;
use open_game_rules_core::Event;

#[component]
fn RootComponent() -> impl IntoView {
    let core = core::new();
    let (view, render) = signal(core.view());
    let (event, set_event) = signal(Event::Start);

    Effect::new(move |_| {
        core::update(&core, event.get(), render);
    });

    view! {
        <section class="box container has-text-centered m-5">
            <p class="is-size-5">{move || view.get().count}</p>
            <div class="buttons section is-centered">
                <button class="button is-primary is-danger"
                    on:click=move |_| set_event.set(Event::Reset)
                >
                    {"Reset"}
                </button>
                <button class="button is-primary is-success"
                    on:click=move |_| set_event.set(Event::Increment)
                >
                    {"Increment"}
                </button>
                <button class="button is-primary is-warning"
                    on:click=move |_| set_event.set(Event::Decrement)
                >
                    {"Decrement"}
                </button>
            </div>

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
