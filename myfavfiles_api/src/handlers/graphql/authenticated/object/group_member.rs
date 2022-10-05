use std::{ops::DerefMut, sync::Arc};

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders};

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

    async fn user(context: &Context) -> Option<Arc<entities::user::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        loaders::cached::find_many_cached(
            context.caches.user.clone(),
            conn,
            Some(vec![self.user_id]),
        )
        .await
        .pop()
    }

    async fn group_roles(context: &Context) -> Vec<Arc<entities::group_role::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let ids_to_load = loaders::cacheless::find_many_ids_related_associative::<
            entities::group_member::Entity,
            entities::group_role::Entity,
            entities::group_member_role::Entity,
        >(conn, self.id)
        .await;

        loaders::cached::find_many_cached(
            context.caches.group_role.clone(),
            conn,
            Some(ids_to_load),
        )
        .await
    }
}
