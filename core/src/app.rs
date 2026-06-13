use crux_core::{App, macros::effect, render::RenderOperation};

use crate::{
    model::{Event, Model},
    view::ViewModel,
};

#[derive(Default)]
pub struct OpenGameRules;

#[effect]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
}

impl App for OpenGameRules {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
    ) -> crux_core::Command<Self::Effect, Self::Event> {
        model.update(event)
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        model.into()
    }
}
