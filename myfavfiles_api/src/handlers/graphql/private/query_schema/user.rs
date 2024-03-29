use std::{ops::DerefMut, sync::Arc};

use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, repository};

#[async_graphql::Object(name = "User")]
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

    async fn group_memberships<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<Arc<entities::group_member::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_member.clone();

        Ok(repository::group_member::group_memberships_by_user_id(conn, cache, self.id).await?)
    }

    async fn platform_roles<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<Arc<entities::platform_role::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.platform_role.clone();

        Ok(repository::platform_role::platform_role_by_user_id(conn, cache, self.id).await?)
    }

    async fn group_file_shares<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<Arc<entities::group_file_share::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_file_share.clone();

        Ok(
            repository::group_file_share::group_file_shares_by_user_id(conn, cache, self.id)
                .await?,
        )
    }

    async fn file_shares<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<Arc<entities::user_file_share::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.user_file_share.clone();

        Ok(repository::user_file_share::user_file_shares_by_user_id(conn, cache, self.id).await?)
    }
}
