use std::sync::Arc;

use myfavfiles_common::config::Config;
use uuid::Uuid;

use crate::{database::entities, database::loaders::Loader, auth::token::Token};

use super::Context;

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    pub fn me() -> String {
        let token = Token {
            sub: Uuid::parse_str("cb8c8187-ecd7-4000-8066-cdc3f8449b35").unwrap(),
            jti: Uuid::parse_str("cb8c8187-ecd7-4000-8066-cdc3f8449b35").unwrap(),
            exp: 1657577027,
        };

        token.encode(&Config::default().jwt_secret).unwrap()
    }

    async fn users(context: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<entities::user::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.user.load_many(context, ids).await
    }

    async fn user(context: &Context, id: Uuid) -> Option<Arc<entities::user::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.user.load_many(context, Some(vec![id])).await.pop()
    }

    async fn groups(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> Vec<Arc<entities::group::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.group.load_many(context, ids).await
    }

    async fn group(context: &Context, id: Uuid) -> Option<Arc<entities::group::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.group.load_many(context, Some(vec![id])).await.pop()
    }

    async fn platform_roles(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> Vec<Arc<entities::platform_role::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.platform_role.load_many(context, ids).await
    }

    async fn platform_role(
        context: &Context,
        id: Uuid,
    ) -> Option<Arc<entities::platform_role::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders
            .platform_role
            .load_many(context, Some(vec![id]))
            .await
            .pop()
    }

    async fn group_roles(
        context: &Context,
        ids: Option<Vec<Uuid>>,
    ) -> Vec<Arc<entities::group_role::Entity>> {
        let mut loaders = context.loaders.lock().await;

        loaders.group_role.load_many(context, ids).await
    }
}
