use std::{ops::DerefMut, sync::Arc};

use chrono::NaiveDateTime;
use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders};

#[graphql_object(Context = Context, name = "GroupFileShare")]
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

    async fn user(&self, context: &Context) -> Option<Arc<entities::user::Entity>> {
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
}
