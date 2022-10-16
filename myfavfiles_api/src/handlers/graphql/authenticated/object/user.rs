use std::{ops::DerefMut, sync::Arc};

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders};

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

    async fn group_memberships(
        &self,
        context: &Context,
    ) -> Vec<Arc<entities::group_member::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let ids_to_load = loaders::cacheless::find_many_ids_related::<
            entities::user::Entity,
            entities::group_member::Entity,
        >(conn, self.id)
        .await;

        loaders::cached::find_many_cached(
            context.caches.group_member.clone(),
            conn,
            Some(ids_to_load),
        )
        .await
    }

    async fn platform_roles(context: &Context) -> Vec<Arc<entities::platform_role::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let ids_to_load = loaders::cacheless::find_many_ids_related_associative::<
            entities::user::Entity,
            entities::platform_role::Entity,
            entities::user_role::Entity,
        >(conn, self.id)
        .await;

        loaders::cached::find_many_cached(
            context.caches.platform_role.clone(),
            conn,
            Some(ids_to_load),
        )
        .await
    }

    async fn group_file_shares(context: &Context) -> Vec<Arc<entities::group_file_share::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let ids_to_load = loaders::cacheless::find_many_ids_related::<
            entities::user::Entity,
            entities::group_file_share::Entity,
        >(conn, self.id)
        .await;

        loaders::cached::find_many_cached(
            context.caches.group_file_share.clone(),
            conn,
            Some(ids_to_load),
        )
        .await
    }

    async fn file_shares(context: &Context) -> Vec<Arc<entities::user_file_share::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let ids_to_load = loaders::cacheless::find_many_ids_related::<
            entities::user::Entity,
            entities::user_file_share::Entity,
        >(conn, self.id)
        .await;

        loaders::cached::find_many_cached(
            context.caches.user_file_share.clone(),
            conn,
            Some(ids_to_load),
        )
        .await
    }
}
