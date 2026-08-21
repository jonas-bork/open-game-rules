use crux_core::{Command, Request, capability::Operation, command::NotificationBuilder};
use facet::Facet;
use serde::{Deserialize, Serialize};

#[derive(Facet, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum NavigationOperation {
    Push(String),
}

impl Operation for NavigationOperation {
    type Output = ();
}

#[must_use]
pub fn push_builder<Effect, Event>(
    path: String,
) -> NotificationBuilder<Effect, Event, impl Future<Output = ()>>
where
    Effect: Send + From<Request<NavigationOperation>> + 'static,
    Event: Send + 'static,
{
    Command::notify_shell(NavigationOperation::Push(path))
}

pub fn push<Effect, Event>(path: String) -> Command<Effect, Event>
where
    Effect: From<Request<NavigationOperation>> + Send + 'static,
    Event: Send + 'static,
{
    push_builder(path).into()
}
