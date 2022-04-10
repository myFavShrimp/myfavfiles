use std::sync::Arc;

use tokio::sync::Mutex;

use crate::AppState;

use self::loaders::Loaders;

pub mod loaders;
pub mod query;

pub struct Context {
    pub app_state: AppState,
    pub loaders: Arc<Mutex<Loaders>>,
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
