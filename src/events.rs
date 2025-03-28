use bevy::prelude::Event;
use spacetimedb_sdk::Error;

#[derive(Event)]
pub struct StdbConnectedEvent;

#[derive(Event)]
pub struct StdbDisonnectedEvent {
    pub err: Option<Error>,
}

#[derive(Event)]
pub struct StdbConnectionErrorEvent {
    pub err: Error,
}

#[derive(Event)]
pub struct InsertEvent<T> {
    pub row: T,
}

#[derive(Event)]
pub struct DeleteEvent<T> {
    pub row: T,
}

#[derive(Event)]
pub struct UpdateEvent<T> {
    pub old: T,
    pub new: T,
}
