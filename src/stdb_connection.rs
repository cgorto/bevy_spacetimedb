use bevy::prelude::Resource;
use spacetimedb_sdk::DbContext;

#[derive(Resource)]
pub struct StdbConnection<T: DbContext> {
    pub conn: T,
}

impl<T: DbContext> StdbConnection<T> {
    pub fn db(&self) -> &T::DbView {
        self.conn.db()
    }

    pub fn reducers(&self) -> &T::Reducers {
        self.conn.reducers()
    }

    pub fn subscribe(&self) -> T::SubscriptionBuilder {
        self.conn.subscription_builder()
    }
}
