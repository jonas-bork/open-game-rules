use crux_core::{
    App,
    macros::effect,
    render::{RenderOperation, render},
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct OpenGameRules;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Increment,
    Decrement,
    Reset,
}

#[derive(Default)]
pub struct Model {
    count: isize,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {
    pub count: String,
}

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
        match event {
            Event::Increment => model.count += 1,
            Event::Decrement => model.count -= 1,
            Event::Reset => model.count = 0,
        }

        render()
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            count: format!("Count is: {}", model.count),
        }
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
