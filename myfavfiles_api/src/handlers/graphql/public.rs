use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{database::PoolConnection, AppState};

pub mod mutation;
pub mod object;
pub mod query;

pub struct Context {
    pub app_state: Arc<AppState>,
    pub database_connection: Arc<Mutex<PoolConnection>>,
}

pub type Root =
    async_graphql::Schema<query::Query, mutation::Mutation, async_graphql::EmptySubscription>;

pub fn create_root() -> Root {
    // std::sync::Arc::new(
    async_graphql::Schema::build(
        query::Query,
        mutation::Mutation,
        async_graphql::EmptySubscription,
    )
    .finish()
    // )
}
