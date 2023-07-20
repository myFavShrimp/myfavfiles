use std::{ops::DerefMut, sync::Arc};

use crate::database::{entities, repository};

use super::{
    object::{group::GroupCreationInput, group_role::GroupRoleCreationInput},
    Context,
};

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_group<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        group: GroupCreationInput,
    ) -> async_graphql::Result<Arc<entities::group::Entity>> {
        let context = context.data::<Context>()?;
        let current_user = context.session_token.sub;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        println!(
            "`{current_user}` wants to create the group `{}`",
            group.name,
        );

        Ok(repository::group::create_group(
            conn,
            context.caches.group.clone(),
            group.name,
            current_user,
        )
        .await?)
    }

    async fn create_group_role<'context>(
        &self,
        context: &async_graphql::Context<'context>,
        group_role: GroupRoleCreationInput,
    ) -> async_graphql::Result<Arc<entities::group_role::Entity>> {
        let context = context.data::<Context>()?;
        let mut lock = context.database_connection.lock().await;
        let conn = lock.deref_mut();

        Ok(repository::group_role::create_group_role(
            conn,
            context.caches.group_role.clone(),
            group_role.name,
            group_role.group_id,
        )
        .await?)
    }
}
