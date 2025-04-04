use bevy::prelude::EventReader;
use spacetimedb_sdk::ReducerEvent;

use crate::{DeleteEvent, InsertEvent, InsertUpdateEvent, UpdateEvent};

/// A type alias for a Bevy event reader for InsertEvent<T>.
pub type ReadInsertEvent<'w, 's, T> = EventReader<'w, 's, InsertEvent<T>>;

/// A type alias for a Bevy event reader for UpdateEvent<T>.
pub type ReadUpdateEvent<'w, 's, T> = EventReader<'w, 's, UpdateEvent<T>>;

/// A type alias for a Bevy event reader for DeleteEvent<T>.
pub type ReadDeleteEvent<'w, 's, T> = EventReader<'w, 's, DeleteEvent<T>>;

/// A type alias for a Bevy event reader for InsertUpdateEvent<T>.
pub type ReadInsertUpdateEvent<'w, 's, T> = EventReader<'w, 's, InsertUpdateEvent<T>>;

/// A type alias for a Bevy event reader for ReducerEvent<T>.
pub type ReadReducerEvent<'w, 's, T> = EventReader<'w, 's, ReducerEvent<T>>;
