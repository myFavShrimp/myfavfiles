use std::{ops::DerefMut, sync::Arc};

use uuid::Uuid;

use super::Context;
use crate::database::{entities, loaders};

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
    async fn me(context: &Context) -> Arc<entities::user::Entity> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        loaders::cached::find_many_cached(
            context.caches.user.clone(),
            conn,
            Some(vec![context.session_token.sub]),
        )
        .await
        .pop()
        .unwrap()
    }

    async fn users(context: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<entities::user::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        loaders::cached::find_many_cached(context.caches.user.clone(), conn, ids).await
    }

    async fn groups(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> Vec<Arc<entities::group::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        loaders::cached::find_many_cached(context.caches.group.clone(), conn, ids).await
    }

    async fn platform_roles(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> Vec<Arc<entities::platform_role::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        loaders::cached::find_many_cached(context.caches.platform_role.clone(), conn, ids).await
    }

    async fn group_roles(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> Vec<Arc<entities::group_role::Entity>> {
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        loaders::cached::find_many_cached(context.caches.group_role.clone(), conn, ids).await
    }
}
