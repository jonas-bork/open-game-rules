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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn renders() {
        let app = OpenGameRules;
        let mut model = Model::default();

        // Check update asked us to `Render`, and only that
        app.update(Event::Reset, &mut model).expect_only_render();
    }

    #[test]
    fn shows_initial_count() {
        let app = OpenGameRules;
        let model = Model::default();

        let actual_view = app.view(&model).count;
        let expected_view = "Count is: 0";

        assert_eq!(actual_view, expected_view);
    }

    #[test]
    fn increments_count() {
        let app = OpenGameRules;
        let mut model = Model::default();

        // Check update asked us to `Render`, and only that
        app.update(Event::Increment, &mut model)
            .expect_only_render();

        let actual_view = app.view(&model).count;
        let expected_view = "Count is: 1";
        assert_eq!(actual_view, expected_view);
    }
}
