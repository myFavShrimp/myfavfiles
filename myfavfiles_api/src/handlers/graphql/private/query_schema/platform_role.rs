use std::{ops::DerefMut, sync::Arc};

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::{database::entities, handlers::graphql::private::data};

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

        let cache = context.caches.user.clone();

        data::user::user_by_platform_role_id(conn, cache, self.id).await
    }
}
