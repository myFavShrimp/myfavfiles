use std::sync::Arc;

use uuid::Uuid;

use crate::{entities, handlers::graphql::loaders::Loadable};

use super::Context;


pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    fn hello(context: &Context) -> String {
        context.app_state.as_ref().config.database_url.clone()
    }

    async fn users(context: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<entities::user::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.user.load_many(context, ids).await
    }

    async fn user(context: &Context, id: Uuid) -> Option<Arc<entities::user::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.user.load_many(context, Some(vec![id])).await.pop()
    }

    async fn groups(context: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<entities::group::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.group.load_many(context, ids).await
    }

    async fn group(context: &Context, id: Uuid) -> Option<Arc<entities::group::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.group.load_many(context, Some(vec![id])).await.pop()
    }
}
