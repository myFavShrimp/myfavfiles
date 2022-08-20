use crate::{database::PoolConnection, AppState};

pub mod query;

pub struct Context {
    pub app_state: AppState,
    pub database_connection: PoolConnection,
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
