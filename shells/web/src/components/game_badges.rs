use leptos::prelude::*;
use phosphor_leptos::Icon;

use crate::components::common::badge::{Badge, BadgeVariant};

#[component]
pub fn game_badges(
    equipment: impl IntoIterator<Item = String> + Send + 'static,
    players: String,
    playing_time: String,
    complexity: String,
    tags: impl IntoIterator<Item = String> + Send + 'static,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <BadgeList>
                // Equipment
                <Element icon=phosphor_leptos::PACKAGE content={equipment.into_iter().next().unwrap()} />

                // Players
                <Element icon=phosphor_leptos::USERS content={players} />

                // Playing time
                <Element icon=phosphor_leptos::CLOCK content={playing_time} />

                // Complexity
                <Element icon=phosphor_leptos::BRAIN content=complexity />
            </BadgeList>
            <BadgeList>
                {
                    tags.into_iter().map(|tag| {
                        view! {
                            <Badge variant=BadgeVariant::SurfaceVariant>{tag.clone()}</Badge>
                        }
                    }).collect::<Vec<_>>()
                }
            </BadgeList>
        </div>
    }
}

#[component]
fn element(icon: phosphor_leptos::IconData, content: String) -> impl IntoView {
    view! {
        <div class="inline-flex flex-row items-center text-sm gap-1 font-medium">
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
