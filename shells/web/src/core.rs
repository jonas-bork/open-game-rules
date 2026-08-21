use std::rc::Rc;

use leptos::prelude::{Update as _, WriteSignal};
use leptos_router::{NavigateOptions, hooks::use_navigate};
use open_game_rules_core::{Effect, NavigationOperation, OpenGameRules, view::ViewModel};

pub type Core = Rc<open_game_rules_core::Core<OpenGameRules>>;
pub type Event = open_game_rules_core::Event;

pub fn new() -> Core {
    Rc::new(open_game_rules_core::Core::new())
}

pub fn update(core: &Core, event: Event, render: WriteSignal<ViewModel>) {
    for effect in &core.process_event(event) {
        process_effect(core, effect, render);
    }
}

pub fn process_effect(core: &Core, effect: &Effect, render: WriteSignal<ViewModel>) {
    match effect {
        Effect::Render(_) => {
            render.update(|view| *view = core.view());
        }
        Effect::Navigate(request) => match &request.operation {
            NavigationOperation::Push(path) => use_navigate()(path, NavigateOptions::default()),
        },
    }
}
