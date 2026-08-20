use leptos::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Secondary,
    Primary,
}

impl BadgeVariant {
    const fn classes(self) -> &'static str {
        match self {
            Self::Primary => "bg-primary-container text-on-primary-container",
            Self::Secondary => "bg-secondary-container text-on-secondary-container",
        }
    }
}

#[component]
pub fn badge(#[prop(optional)] variant: BadgeVariant, children: Children) -> impl IntoView {
    let class = format!(
        "inline-flex gap-1.5 items-center rounded-full px-3 py-1 text-sm font-medium {}",
        variant.classes()
    );

    view! {
        <div class=class>
            {children()}
        </div>
    }
}
