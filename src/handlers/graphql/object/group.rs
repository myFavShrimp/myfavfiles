use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders::Loadable};

#[graphql_object(Context = Context, name = "Group")]
impl entities::group::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn user(context: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<entities::user::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.user.load_many(context, ids).await
    }
}
