use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    auth::token::Token,
    database::{cache::Caches, PoolConnection},
    AppState,
};

pub mod mutation_schema;
mod object;
pub mod query_schema;

pub struct Context {
    pub app_state: AppState,
    pub database_connection: Arc<Mutex<PoolConnection>>,
    pub caches: Caches,
    pub session_token: Token,
}

impl juniper::Context for Context {}

pub type Root = std::sync::Arc<
    juniper::RootNode<
        'static,
        query_schema::Query,
        mutation_schema::Mutation,
        juniper::EmptySubscription<Context>,
    >,
>;

pub fn create_root() -> Root {
    std::sync::Arc::new(juniper::RootNode::new(
        query_schema::Query,
        mutation_schema::Mutation,
        juniper::EmptySubscription::<Context>::new(),
    ))
}
