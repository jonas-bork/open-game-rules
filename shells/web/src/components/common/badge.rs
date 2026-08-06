use leptos::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Gray,
    Blue,
}

impl BadgeVariant {
    const fn classes(self) -> &'static str {
        match self {
            Self::Blue => "bg-blue-100 text-blue-800",
            Self::Gray => "bg-gray-200 text-gray-800",
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
