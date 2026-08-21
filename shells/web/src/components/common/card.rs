use leptos::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CardVariant {
    // Elevated,
    // Filled,
    Outlined,
}

impl CardVariant {
    const fn classes(self) -> &'static str {
        match self {
            Self::Outlined => "border border-outline-variant bg-surface text-on-surface",
        }
    }

    const fn clickable_classes(self) -> &'static str {
        match self {
            Self::Outlined => {
                "hover:bg-on-surface/[8%] focus-visible:bg-on-surface/[10%] focus-visible:outline focus-visible:outline-[3px] focus-visible:outline-offset-2 focus-visible:outline-secondary active:bg-on-surface/10"
            }
        }
    }
}

#[component]
pub fn card(
    variant: CardVariant,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let mut class = format!(
        "flex flex-col items-start p-4 rounded-xl {}",
        variant.classes()
    );

    let is_clickable = on_click.is_some();
    if is_clickable {
        class.push_str(&format!(
            " cursor-pointer transition-all duration-200 ease-in-out {}",
            variant.clickable_classes()
        ));
    }

    view! {
        <div
            class=class
            on:click=move |_| {
                if let Some(on_click) = on_click {
                    on_click.run(());
                }
            }
            on:keydown=move |e| {
                if is_clickable && (e.key() == "Enter" || e.key() == " ") {
                    // Prevent the default browser behavior (which scrolls the page down on Spacebar)
                    e.prevent_default();

                    if let Some(cb) = on_click {
                        cb.run(());
                    }
                }
            }            role=if is_clickable { "button" } else { "article" }
            tabindex=if is_clickable { "0" } else { "-1" }
        >
            {children()}
        </div>
    }
}
