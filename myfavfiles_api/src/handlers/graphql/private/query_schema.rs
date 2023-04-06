use std::{ops::DerefMut, sync::Arc};

use juniper::FieldResult;
use uuid::Uuid;

use super::Context;
use crate::{
    database::{entities, loaders},
    handlers::graphql::private::data,
};

mod group;
mod group_file_share;
mod group_member;
mod group_role;
mod platform_role;
mod user;
mod user_file_share;

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    async fn me(context: &Context) -> FieldResult<Arc<entities::user::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        Ok(
            data::user::user_by_id(conn, context.caches.user.clone(), context.session_token.sub)
                .await?
                .unwrap(),
        )
    }

    async fn users(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> FieldResult<Vec<Arc<entities::user::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        Ok(data::user::users_by_ids(conn, context.caches.user.clone(), ids).await?)
    }

    async fn groups(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> FieldResult<Vec<Arc<entities::group::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        Ok(data::group::groups_by_ids(conn, context.caches.group.clone(), ids).await?)
    }

    async fn platform_roles(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> FieldResult<Vec<Arc<entities::platform_role::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        Ok(data::platform_role::platform_roles_by_ids(
            conn,
            context.caches.platform_role.clone(),
            ids,
        )
        .await?)
    }

    async fn group_roles(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> FieldResult<Vec<Arc<entities::group_role::Entity>>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_role.clone();

        Ok(data::group_role::group_roles_by_ids(conn, cache, ids).await?)
    }
}
