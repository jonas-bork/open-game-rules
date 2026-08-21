use leptos::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    SurfaceVariant,
}

impl BadgeVariant {
    const fn classes(self) -> &'static str {
        match self {
            Self::SurfaceVariant => "bg-surface-variant text-on-surface-variant",
        }
    }
}

#[component]
pub fn badge(
    #[prop(optional)] variant: BadgeVariant,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "inline-flex gap-1.5 items-center rounded-full px-3 py-1 text-sm font-medium {}",
        variant.classes()
    );

    view! {
        <div class=class style=style>
            {children()}
        </div>
    }
}
