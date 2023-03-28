use std::{ops::DerefMut, sync::Arc};

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::{database::entities, handlers::graphql::private::data};

#[graphql_object(Context = Context, name = "GroupMember")]
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

    async fn group(&self, context: &Context) -> Option<Arc<entities::group::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group.clone();

        data::group::group_by_id(conn, cache, self.group_id).await
    }

    async fn user(context: &Context) -> Option<Arc<entities::user::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.user.clone();

        data::user::user_by_id(conn, cache, self.user_id).await
    }

    async fn group_roles(context: &Context) -> Vec<Arc<entities::group_role::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_role.clone();

        data::group_role::group_roles_by_group_member_id(conn, cache, self.id).await
    }
}
