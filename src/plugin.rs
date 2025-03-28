use std::sync::mpsc::{Sender, channel};

use bevy::app::{App, Plugin};
use spacetimedb_sdk::{DbContext, Table, TableWithPrimaryKey};

use crate::{
    DeleteEvent, InsertEvent, StdbConnectedEvent, StdbConnection, StdbConnectionErrorEvent,
    StdbDisonnectedEvent, UpdateEvent, channel_receiver::AppExtensions,
};

pub type FnBuildConnection<T> = fn(
    Sender<StdbConnectedEvent>,
    Sender<StdbDisonnectedEvent>,
    Sender<StdbConnectionErrorEvent>,
    &mut App,
) -> T;
pub type FnRegisterCallbacks<T> = fn(&StdbPlugin<T>, &mut App, &<T as DbContext>::DbView);

pub struct StdbPlugin<T: DbContext> {
    connection_builder: Option<FnBuildConnection<T>>,
    register_events: Option<FnRegisterCallbacks<T>>,
}

impl<TConnection: DbContext> StdbPlugin<TConnection> {
    pub fn with_connection(mut self, build_connection: FnBuildConnection<TConnection>) -> Self {
        self.connection_builder = Some(build_connection);
        self
    }

    pub fn with_events(mut self, register_callbacks: FnRegisterCallbacks<TConnection>) -> Self {
        self.register_events = Some(register_callbacks);
        self
    }

    pub fn on_insert<TRow>(&self, app: &mut App, table: impl Table<Row = TRow>) -> &Self
    where
        TRow: Send + Sync + Clone + 'static,
    {
        let (send, recv) = channel::<InsertEvent<TRow>>();
        app.add_event_channel(recv);

        table.on_insert(move |_ctx, row| {
            let event = InsertEvent { row: row.clone() };
            send.send(event).unwrap();
        });

        self
    }

    pub fn on_delete<TRow>(&self, app: &mut App, table: impl Table<Row = TRow>) -> &Self
    where
        TRow: Send + Sync + Clone + 'static,
    {
        let (send, recv) = channel::<DeleteEvent<TRow>>();
        app.add_event_channel(recv);

        table.on_delete(move |_ctx, row| {
            let event = DeleteEvent { row: row.clone() };
            send.send(event).unwrap();
        });

        self
    }

    pub fn on_update<TRow, TTable>(&self, app: &mut App, table: TTable) -> &Self
    where
        TRow: Send + Sync + Clone + 'static,
        TTable: Table<Row = TRow> + TableWithPrimaryKey<Row = TRow>,
    {
        let (send, recv) = channel::<UpdateEvent<TRow>>();
        app.add_event_channel(recv);

        table.on_update(move |_ctx, old, new| {
            let event = UpdateEvent {
                old: old.clone(),
                new: new.clone(),
            };
            send.send(event).unwrap();
        });

        self
    }
}

impl<T: DbContext> Default for StdbPlugin<T> {
    fn default() -> Self {
        Self {
            connection_builder: None,
            register_events: None,
        }
    }
}

impl<T: DbContext + Send + Sync + 'static> Plugin for StdbPlugin<T> {
    fn build(&self, app: &mut App) {
        let (send_connected, recv_connected) = channel::<StdbConnectedEvent>();
        let (send_disconnected, recv_disconnected) = channel::<StdbDisonnectedEvent>();
        let (send_connect_error, recv_connect_error) = channel::<StdbConnectionErrorEvent>();

        app.add_event_channel::<StdbConnectionErrorEvent>(recv_connect_error)
            .add_event_channel::<StdbConnectedEvent>(recv_connected)
            .add_event_channel::<StdbDisonnectedEvent>(recv_disconnected);

        let conn_builder = self
            .connection_builder
            .expect("Connection builder is not set, use with_connection() method");
        let conn = conn_builder(send_connected, send_disconnected, send_connect_error, app);

        if let Some(register_callbacks) = self.register_events {
            register_callbacks(self, app, conn.db());
        }

        app.insert_resource(StdbConnection { conn });
    }
}
