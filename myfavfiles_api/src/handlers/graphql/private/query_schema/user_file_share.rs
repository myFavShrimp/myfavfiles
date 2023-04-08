use std::{ops::DerefMut, sync::Arc};

use chrono::NaiveDateTime;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, repository};

#[async_graphql::Object(name = "UserFileShare")]
impl entities::user_file_share::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn user_id(&self) -> Uuid {
        self.user_id
    }

    async fn expiration(&self) -> NaiveDateTime {
        self.expiration
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
}
