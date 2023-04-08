use std::{ops::DerefMut, sync::Arc};

use chrono::NaiveDateTime;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, repository};

#[async_graphql::Object(name = "GroupFileShare")]
impl entities::group_file_share::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn user_id(&self) -> Uuid {
        self.user_id
    }

    async fn group_id(&self) -> Uuid {
        self.group_id
    }

    async fn expiration(&self) -> NaiveDateTime {
        self.expiration
    }

    async fn group<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Option<Arc<entities::group::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let db_connection = lock.deref_mut();

        let cache = context.caches.group.clone();

        Ok(repository::group::group_by_id(db_connection, cache, self.id).await?)
    }

    async fn user<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Option<Arc<entities::user::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let db_connection = lock.deref_mut();

        let cache = context.caches.user.clone();

        Ok(repository::user::user_by_id(db_connection, cache, self.user_id).await?)
    }
}
