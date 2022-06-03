use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders::{LoadableRelationOneToMany, LoadableRelationManyToMany}};

#[graphql_object(Context = Context, name = "User")]
impl entities::user::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn is_admin(&self) -> bool {
        self.is_admin
    }

    async fn group_member(context: &Context) -> Vec<Arc<entities::group_member::Entity>> {
        let mut loaders = context.loaders.lock().await;

        LoadableRelationOneToMany::<entities::user::Columns>::load_many_related(
            &mut loaders.group_member,
            context,
            vec![self.id],
        )
        .await
    }

    async fn platform_role(context: &Context) -> Vec<Arc<entities::platform_role::Entity>> {
        let mut loaders = context.loaders.lock().await;

        LoadableRelationManyToMany::<entities::user::Columns>::load_many_related(
            &mut loaders.platform_role,
            context,
            vec![self.id],
        )
        .await
    }
}
