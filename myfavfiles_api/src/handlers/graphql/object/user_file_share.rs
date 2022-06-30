use std::sync::Arc;

use chrono::NaiveDateTime;
use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders::Loader};

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

    async fn user(context: &Context) -> Vec<Arc<entities::user::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders
            .user
            .load_many(context, Some(vec![self.user_id]))
            .await
    }
}
