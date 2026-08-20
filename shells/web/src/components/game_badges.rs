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
    let special_badge_variant = BadgeVariant::Primary;
    view! {
        <div class="flex flex-col gap-1">
            <BadgeList>
                // Equipment
                <Badge variant=special_badge_variant>
                    <Icon icon=phosphor_leptos::PACKAGE size="18px" />
                    <span>{equipment.into_iter().next().unwrap()}</span>
                </Badge>

                // Players
                <Badge variant=special_badge_variant>
                    <Icon icon=phosphor_leptos::USERS size="18px" />
                    <span>{players}</span>
                </Badge>

                // Playing time
                <Badge variant=special_badge_variant>
                    <Icon icon=phosphor_leptos::CLOCK size="18px" />
                    <span>{playing_time}</span>
                </Badge>

                // Complexity
                <Badge variant=special_badge_variant>
                    <Icon icon=phosphor_leptos::BRAIN size="18px" />
                    <span>{complexity}</span>
                </Badge>
            </BadgeList>
            <BadgeList>
                {
                    tags.into_iter().map(|tag| {
                        view! {
                            <Badge>{tag.clone()}</Badge>
                        }
                    }).collect::<Vec<_>>()
                }
            </BadgeList>
        </div>
    }
}

#[component]
pub fn badge_list(children: Children) -> impl IntoView {
    let classes = "flex flex-row flex-wrap gap-x-2 gap-y-1";

    view! {
        <div class=classes >
            {children()}
        </div>
    }
}
