use leptos::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    // Filled,
    // Tonal
    // Outlined,
    Standard,
}

impl Color {
    const fn classes(self) -> &'static str {
        match self {
            Self::Standard => {
                "text-on-surface-variant hover:bg-on-surface-variant/[8%] focus-visible:bg-on-surface-variant/[10%] focus-visible:outline focus-visible:outline-[3px] focus-visible:outline-offset-2 focus-visible:outline-secondary active:bg-on-surface-variant/10"
            }
        }
    }
}

#[component]
pub fn icon_button(
    color: Color,
    #[prop(into)] on_click: Callback<()>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "flex items-center justify-center p-1 rounded-full transition-color duration-200 cursor-pointer {}",
        color.classes()
    );

    view! {
        <button
            class=class
            style=style
            on:click=move |_| {
                on_click.run(());
            }
        >
            {children()}
        </button>
    }
}
