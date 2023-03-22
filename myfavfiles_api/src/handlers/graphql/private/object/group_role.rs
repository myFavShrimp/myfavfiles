use std::{ops::DerefMut, sync::Arc};

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders};

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
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let ids_to_load = loaders::cacheless::find_many_ids_related_associative::<
            entities::group_role::Entity,
            entities::group_member::Entity,
            entities::group_member_role::Entity,
        >(conn, self.id)
        .await;

        loaders::cached::find_many_cached(
            context.caches.group_member.clone(),
            conn,
            Some(ids_to_load),
        )
        .await
    }

    async fn group(&self, context: &Context) -> Option<Arc<entities::group::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        loaders::cached::find_many_cached(
            context.caches.group.clone(),
            conn,
            Some(vec![self.group_id]),
        )
        .await
        .pop()
    }
}
