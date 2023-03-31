use std::{ops::DerefMut, sync::Arc};

use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

use super::super::Context;
use crate::{database::entities, handlers::graphql::private::data};

#[graphql_object(Context = Context, name = "GroupRole")]
impl entities::group_role::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn group_id(&self) -> Uuid {
        self.group_id
    }

    async fn permissions(&self) -> Option<Vec<entities::group_role::GroupRolePermission>> {
        self.permissions.clone()
    }

    async fn group_members(
        context: &Context,
    ) -> FieldResult<Vec<Arc<entities::group_member::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_member.clone();

        Ok(data::group_member::group_memberships_by_group_role_id(conn, cache, self.id).await?)
    }

    async fn group(&self, context: &Context) -> FieldResult<Option<Arc<entities::group::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group.clone();

        Ok(data::group::group_by_id(conn, cache, self.group_id).await?)
    }
}
