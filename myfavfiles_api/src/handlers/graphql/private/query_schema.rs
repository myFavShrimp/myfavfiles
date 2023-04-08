use std::{ops::DerefMut, sync::Arc};

use uuid::Uuid;

use super::Context;
use crate::database::{entities, repository};

mod group;
mod group_file_share;
mod group_member;
mod group_role;
mod platform_role;
mod user;
mod user_file_share;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn me<'context>(
        &self,
        context: &async_graphql::Context<'context>,
    ) -> async_graphql::Result<Arc<entities::user::Entity>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        Ok(repository::user::user_by_id(
            conn,
            context.caches.user.clone(),
            context.session_token.sub,
        )
        .await?
        .unwrap())
    }

    async fn users<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        ids: Option<Vec<Uuid>>,
    ) -> async_graphql::Result<Vec<Arc<entities::user::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        Ok(repository::user::users_by_ids(conn, context.caches.user.clone(), ids).await?)
    }

    async fn groups<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        ids: Option<Vec<Uuid>>,
    ) -> async_graphql::Result<Vec<Arc<entities::group::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        Ok(repository::group::groups_by_ids(conn, context.caches.group.clone(), ids).await?)
    }

    async fn platform_roles<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        ids: Option<Vec<Uuid>>,
    ) -> async_graphql::Result<Vec<Arc<entities::platform_role::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        Ok(repository::platform_role::platform_roles_by_ids(
            conn,
            context.caches.platform_role.clone(),
            ids,
        )
        .await?)
    }

    async fn group_roles<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        ids: Option<Vec<Uuid>>,
    ) -> async_graphql::Result<Vec<Arc<entities::group_role::Entity>>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        let cache = context.caches.group_role.clone();

        Ok(repository::group_role::group_roles_by_ids(conn, cache, ids).await?)
    }
}
