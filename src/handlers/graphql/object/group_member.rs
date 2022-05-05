use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders::{Loadable, LoadableRelationOneToOne}};

#[graphql_object(Context = Context, name = "GroupMember")]
impl entities::group_member::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn user_id(&self) -> Uuid {
        self.user_id
    }

    async fn group_id(&self) -> Uuid {
        self.group_id
    }

    async fn is_admin(&self) -> bool {
        self.is_admin
    }

    async fn group(context: &Context) -> Vec<Arc<entities::group::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.group.load_many(context, Some(vec![self.group_id])).await
    }

    async fn user(context: &Context) -> Vec<Arc<entities::user::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.user.load_many(context, Some(vec![self.user_id])).await
    }
}
