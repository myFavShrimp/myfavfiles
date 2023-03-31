use std::{ops::DerefMut, sync::Arc};

use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

use super::super::Context;
use crate::{database::entities, handlers::graphql::private::data};

#[graphql_object(Context = Context, name = "User")]
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

    async fn group_memberships(
        &self,
        context: &Context,
    ) -> FieldResult<Vec<Arc<entities::group_member::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_member.clone();

        Ok(data::group_member::group_memberships_by_user_id(conn, cache, self.id).await?)
    }

    async fn platform_roles(
        context: &Context,
    ) -> FieldResult<Vec<Arc<entities::platform_role::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.platform_role.clone();

        Ok(data::platform_role::platform_role_by_user_id(conn, cache, self.id).await?)
    }

    async fn group_file_shares(
        context: &Context,
    ) -> FieldResult<Vec<Arc<entities::group_file_share::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_file_share.clone();

        Ok(data::group_file_share::group_file_shares_by_user_id(conn, cache, self.id).await?)
    }

    async fn file_shares(
        context: &Context,
    ) -> FieldResult<Vec<Arc<entities::user_file_share::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.user_file_share.clone();

        Ok(data::user_file_share::user_file_shares_by_user_id(conn, cache, self.id).await?)
    }
}
