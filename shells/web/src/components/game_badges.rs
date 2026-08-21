use leptos::prelude::*;
use phosphor_leptos::Icon;

use crate::components::common::badge::{Badge, BadgeVariant};

#[component]
pub fn game_badges(
    game_id: String,
    equipment: impl IntoIterator<Item = String> + Send + 'static,
    players: String,
    playing_time: String,
    complexity: String,
    tags: impl IntoIterator<Item = String> + Send + 'static,
) -> impl IntoView {
    let transition_name_style = format!("view-transition-name: game-badges-{}", game_id);
    let game_id_for_tags = game_id.clone();
    view! {
        <div class="flex flex-col gap-2" style=transition_name_style>
            <BadgeList>
                // Equipment
                <Element id=format!("{game_id}-equipment") icon=phosphor_leptos::PACKAGE content={equipment.into_iter().next().unwrap()} />

                // Players
                <Element id=format!("{game_id}-players") icon=phosphor_leptos::USERS content={players} />

                // Playing time
                <Element id=format!("{game_id}-playing-time") icon=phosphor_leptos::CLOCK content={playing_time} />

                // Complexity
                <Element id=format!("{game_id}-complexity") icon=phosphor_leptos::BRAIN content=complexity />
            </BadgeList>
            <BadgeList>
                {
                    tags.into_iter().map(move |tag| {
                        let transition_name_style = format!("view-transition-name: game-badges-tags-{game_id_for_tags}-{tag}");
                        view! {
                            <Badge variant=BadgeVariant::SurfaceVariant style=transition_name_style>{tag}</Badge>
                        }
                    }).collect::<Vec<_>>()
                }
            </BadgeList>
        </div>
    }
}

#[component]
fn element(id: String, icon: phosphor_leptos::IconData, content: String) -> impl IntoView {
    let transition_name_style = format!("view-transition-name: game-badges-{id}");
    view! {
        <div class="inline-flex flex-row items-center text-sm gap-1 font-medium" style=transition_name_style>
            <Icon icon={icon} size="18px" />
            <span>{content}</span>
        </div>
    }
}

#[component]
fn badge_list(children: Children) -> impl IntoView {
    let classes = "flex flex-row flex-wrap gap-x-3 gap-y-1";

    view! {
        <div class=classes >
            {children()}
        </div>
    }
}
