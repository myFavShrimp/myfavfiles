use std::{ops::DerefMut, sync::Arc};

use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, repository};

#[async_graphql::Object(name = "PlatformRole")]
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

    async fn users<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<Arc<entities::user::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.user.clone();

        Ok(repository::user::user_by_platform_role_id(conn, cache, self.id).await?)
    }
}
