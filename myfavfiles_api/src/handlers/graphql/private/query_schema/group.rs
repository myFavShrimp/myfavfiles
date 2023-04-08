use std::{ops::DerefMut, sync::Arc};

use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, repository};

#[async_graphql::Object(name = "Group")]
impl entities::group::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn group_members<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<Arc<entities::group_member::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_member.clone();

        Ok(repository::group_member::group_memberships_by_group_id(conn, cache, self.id).await?)
    }

    async fn group_roles<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<Arc<entities::group_role::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_role.clone();

        Ok(repository::group_role::group_roles_by_group_id(conn, cache, self.id).await?)
    }

    async fn file_shares<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<Arc<entities::group_file_share::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_file_share.clone();

        Ok(
            repository::group_file_share::group_file_shares_by_group_id(conn, cache, self.id)
                .await?,
        )
    }
}
