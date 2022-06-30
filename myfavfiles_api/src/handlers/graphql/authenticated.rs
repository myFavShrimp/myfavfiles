use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{database::loaders::Loaders, AppState};

mod object;
pub mod query;

pub struct Context {
    pub app_state: AppState,
    pub loaders: Arc<Mutex<Loaders>>,
}

impl Context {
    pub async fn database_connection(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, sqlx::Error> {
        self.app_state.clone().database_connection.acquire().await
    }
}

impl juniper::Context for Context {}

pub type Root = std::sync::Arc<
    juniper::RootNode<
        'static,
        query::Query,
        juniper::EmptyMutation<Context>,
        juniper::EmptySubscription<Context>,
    >,
>;

pub fn create_root() -> Root {
    std::sync::Arc::new(juniper::RootNode::new(
        query::Query,
        juniper::EmptyMutation::<Context>::new(),
        juniper::EmptySubscription::<Context>::new(),
    ))
}
