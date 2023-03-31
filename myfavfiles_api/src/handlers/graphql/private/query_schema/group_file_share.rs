use std::{ops::DerefMut, sync::Arc};

use chrono::NaiveDateTime;
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

use super::super::Context;
use crate::{database::entities, handlers::graphql::private::data};

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

    async fn group(&self, context: &Context) -> FieldResult<Option<Arc<entities::group::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let db_connection = lock.deref_mut();

        let cache = context.caches.group.clone();

        Ok(data::group::group_by_id(db_connection, cache, self.id).await?)
    }

    async fn user(&self, context: &Context) -> FieldResult<Option<Arc<entities::user::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let db_connection = lock.deref_mut();

        let cache = context.caches.user.clone();

        Ok(data::user::user_by_id(db_connection, cache, self.user_id).await?)
    }
}
