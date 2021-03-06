use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{entities, loaders::LoadableRelationOneToMany};

#[graphql_object(Context = Context, name = "Group")]
impl entities::group::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn group_members(context: &Context) -> Vec<Arc<entities::group_member::Entity>> {
        let mut loaders = context.loaders.lock().await;

        LoadableRelationOneToMany::<entities::group::Columns>::load_many_related(
            &mut loaders.group_member,
            context,
            vec![self.id],
        )
        .await
    }

    async fn group_roles(context: &Context) -> Vec<Arc<entities::group_role::Entity>> {
        let mut loaders = context.loaders.lock().await;

        LoadableRelationOneToMany::<entities::group::Columns>::load_many_related(
            &mut loaders.group_role,
            context,
            vec![self.id],
        )
        .await
    }

    async fn file_shares(context: &Context) -> Vec<Arc<entities::group_file_share::Entity>> {
        let mut loaders = context.loaders.lock().await;

        LoadableRelationOneToMany::<entities::group::Columns>::load_many_related(
            &mut loaders.group_file_share,
            context,
            vec![self.id],
        )
        .await
    }
}
