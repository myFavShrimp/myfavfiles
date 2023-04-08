use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    auth::token::Token,
    database::{Caches, PoolConnection},
    AppState,
};

pub mod mutation_schema;
mod object;
pub mod query_schema;

pub struct Context {
    pub app_state: Arc<AppState>,
    pub database_connection: Arc<Mutex<PoolConnection>>,
    pub caches: Caches,
    pub session_token: Token,
}

pub type Root = async_graphql::Schema<
    query_schema::Query,
    mutation_schema::Mutation,
    async_graphql::EmptySubscription,
>;

pub fn create_root() -> Root {
    // std::sync::Arc::new(
    async_graphql::Schema::build(
        query_schema::Query,
        mutation_schema::Mutation,
        async_graphql::EmptySubscription,
    )
    .finish()
    // )
}
