use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{
    entities,
    loaders::{LoadableRelationManyToMany, Loader},
};

#[graphql_object(Context = Context, name = "GroupRole")]
impl entities::group_role::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn group_id(&self) -> Uuid {
        self.group_id
    }

    async fn permissions(&self) -> Option<Vec<entities::group_role::GroupRolePermission>> {
        self.permissions.clone()
    }

    async fn group_members(context: &Context) -> Vec<Arc<entities::group_member::Entity>> {
        let mut loaders = context.loaders.lock().await;

        LoadableRelationManyToMany::<entities::group_role::Columns>::load_many_related(
            &mut loaders.group_member,
            context,
            vec![self.id],
        )
        .await
    }

    async fn group(context: &Context) -> Arc<entities::group::Entity> {
        let mut loaders = context.loaders.lock().await;

        loaders
            .group
            .load_many(context, Some(vec![self.group_id]))
            .await
            .pop()
            .expect("GroupRole has no associated Group")
    }
}
