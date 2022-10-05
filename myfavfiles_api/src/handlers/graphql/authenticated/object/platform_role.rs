use std::{ops::DerefMut, sync::Arc};

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders};

#[graphql_object(Context = Context, name = "PlatformRole")]
impl entities::platform_role::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn permissions(&self) -> Vec<entities::platform_role::PlatformRolePermission> {
        self.permissions.clone()
    }

    async fn users(context: &Context) -> Vec<Arc<entities::user::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let ids_to_load = loaders::cacheless::find_many_ids_related_associative::<
            entities::platform_role::Entity,
            entities::user::Entity,
            entities::user_role::Entity,
        >(conn, self.id)
        .await;

        loaders::cached::find_many_cached(context.caches.user.clone(), conn, Some(ids_to_load))
            .await
    }
}
