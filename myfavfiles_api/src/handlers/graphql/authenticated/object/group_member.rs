use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders::{Loader, LoadableRelationManyToMany}};

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

    async fn group(context: &Context) -> Option<Arc<entities::group::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders
            .group
            .load_many(context, Some(vec![self.group_id]))
            .await
            .pop()
    }

    async fn user(context: &Context) -> Option<Arc<entities::user::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders
            .user
            .load_many(context, Some(vec![self.user_id]))
            .await
            .pop()
    }

    async fn group_roles(context: &Context) -> Vec<Arc<entities::group_role::Entity>> {
        let mut loaders = context.loaders.lock().await;

        LoadableRelationManyToMany::<entities::group_member::Columns>::load_many_related(
            &mut loaders.group_role,
            context,
            vec![self.id],
        )
        .await
    }
}
