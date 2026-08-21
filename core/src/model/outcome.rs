use crux_core::Command;

use crate::Effect;

pub struct Started<S, Event> {
    pub state: S,
    pub command: Command<Effect, Event>,
}

impl<S, Event> Started<S, Event> {
    pub const fn new(state: S, command: Command<Effect, Event>) -> Self {
        Self { state, command }
    }

    pub fn into_parts(self) -> (S, Command<Effect, Event>) {
        (self.state, self.command)
    }

    pub fn map_event<NewEvent>(
        self,
        f: impl Fn(Event) -> NewEvent + Send + Sync + 'static,
    ) -> Started<S, NewEvent>
    where
        Event: Send + Unpin + 'static,
        NewEvent: Send + Unpin + 'static,
    {
        Started {
            state: self.state,
            command: self.command.map_event(f),
        }
    }
}

#[cfg(test)]
impl<S, Event> Started<S, Event> {
    pub fn into_value(self) -> S {
        self.state
    }
}
