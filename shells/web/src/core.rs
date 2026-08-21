use std::rc::Rc;

use leptos::prelude::{Update as _, WriteSignal};
use leptos_router::{NavigateOptions, hooks::use_navigate};
use open_game_rules_core::{Effect, NavigationOperation, OpenGameRules, view::ViewModel};

pub type Core = Rc<open_game_rules_core::Core<OpenGameRules>>;
pub type Event = open_game_rules_core::Event;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    type DocumentExt;

    #[wasm_bindgen(method, getter, js_name = startViewTransition)]
    fn has_start_view_transition(this: &DocumentExt) -> JsValue;

    #[wasm_bindgen(method, js_name = startViewTransition)]
    fn start_view_transition(this: &DocumentExt, callback: &js_sys::Function);
}

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
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::JsCast;
                let document = web_sys::window().unwrap().document().unwrap();
                let doc_ext: &DocumentExt = document.unchecked_ref();

                if !doc_ext.has_start_view_transition().is_undefined() {
                    let core_clone = Rc::clone(core);

                    let cb = Closure::once_into_js(move || {
                        render.update(|view| *view = core_clone.view());
                    });

                    doc_ext.start_view_transition(cb.unchecked_ref());
                } else {
                    render.update(|view| *view = core.view());
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                render.update(|view| *view = core.view());
            }
        }
        Effect::Navigate(request) => match &request.operation {
            NavigationOperation::Push(path) => use_navigate()(path, NavigateOptions::default()),
        },
    }
}
