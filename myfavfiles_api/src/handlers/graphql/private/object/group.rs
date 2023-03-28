use std::{ops::DerefMut, sync::Arc};

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::{database::entities, handlers::graphql::private::data};

#[graphql_object(Context = Context, name = "Group")]
impl entities::group::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn group_members(&self, context: &Context) -> Vec<Arc<entities::group_member::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_member.clone();

        data::group_member::group_memberships_by_group_id(conn, cache, self.id).await
    }

    async fn group_roles(&self, context: &Context) -> Vec<Arc<entities::group_role::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_role.clone();

        data::group_role::group_roles_by_group_id(conn, cache, self.id).await
    }

    async fn file_shares(context: &Context) -> Vec<Arc<entities::group_file_share::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_file_share.clone();

        data::group_file_share::group_file_shares_by_group_id(conn, cache, self.id).await
    }
}
