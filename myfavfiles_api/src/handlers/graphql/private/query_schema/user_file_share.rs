use std::{ops::DerefMut, sync::Arc};

use chrono::NaiveDateTime;
use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::{database::entities, handlers::graphql::private::data};

#[graphql_object(Context = Context, name = "UserFileShare")]
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

    async fn user(context: &Context) -> Option<Arc<entities::user::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.user.clone();

        data::user::user_by_id(conn, cache, self.user_id).await
    }
}