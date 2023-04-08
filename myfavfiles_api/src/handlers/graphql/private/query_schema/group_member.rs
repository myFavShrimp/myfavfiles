use std::{ops::DerefMut, sync::Arc};

use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, repository};

#[async_graphql::Object(name = "GroupMember")]
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

    async fn group<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Option<Arc<entities::group::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group.clone();

        Ok(repository::group::group_by_id(conn, cache, self.group_id).await?)
    }

    async fn user<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Option<Arc<entities::user::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.user.clone();

        Ok(repository::user::user_by_id(conn, cache, self.user_id).await?)
    }

    async fn group_roles<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Vec<Arc<entities::group_role::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_role.clone();

        Ok(repository::group_role::group_roles_by_group_member_id(conn, cache, self.id).await?)
    }
}
