use crux_core::Command;

use crate::Effect;

#[derive(Debug)]
pub enum Status<S, T> {
    Continue(S),
    Complete(T),
}
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

pub struct Outcome<S, T, Event> {
    pub status: Status<S, T>,
    pub command: Command<Effect, Event>,
}

impl<S, T, Event> Outcome<S, T, Event> {
    pub const fn continuing(state: S, command: Command<Effect, Event>) -> Self {
        Self {
            status: Status::Continue(state),
            command,
        }
    }

    pub const fn complete(value: T, command: Command<Effect, Event>) -> Self {
        Self {
            status: Status::Complete(value),
            command,
        }
    }

    pub fn into_parts(self) -> (Status<S, T>, Command<Effect, Event>) {
        (self.status, self.command)
    }

    pub fn map_event<NewEvent>(
        self,
        f: impl Fn(Event) -> NewEvent + Send + Sync + 'static,
    ) -> Outcome<S, T, NewEvent>
    where
        Event: Send + Unpin + 'static,
        NewEvent: Send + Unpin + 'static,
    {
        Outcome {
            status: self.status,
            command: self.command.map_event(f),
        }
    }
}

#[cfg(test)]
pub(crate) struct Asserted<V, Event> {
    pub value: V,
    pub command: Command<Effect, Event>,
}

#[cfg(test)]
impl<V, Event> Asserted<V, Event> {
    pub fn into_value(self) -> V {
        self.value
    }

    pub fn into_command(self) -> Command<Effect, Event> {
        self.command
    }

    pub fn into_parts(self) -> (V, Command<Effect, Event>) {
        (self.value, self.command)
    }
}

#[cfg(test)]
impl<S: std::fmt::Debug, T: std::fmt::Debug, Event> Outcome<S, T, Event> {
    pub fn expect_continue(self) -> Asserted<S, Event> {
        let Status::Continue(state) = self.status else {
            panic!("expected status to be 'continue'")
        };
        Asserted {
            value: state,
            command: self.command,
        }
    }

    pub fn expect_complete(self) -> Asserted<T, Event> {
        let Status::Complete(value) = self.status else {
            panic!("expected status to be 'complete'")
        };
        Asserted {
            value,
            command: self.command,
        }
    }
}
