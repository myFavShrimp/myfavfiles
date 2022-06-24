use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{
    entities,
    loaders::LoadableRelationManyToMany,
};

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

    async fn permissions(&self) -> Vec<entities::group_role::GroupRolePermission> {
        self.permissions.clone()
    }

    async fn group_members(context: &Context) -> Vec<Arc<entities::group_member::Entity>> {
        let mut loaders = context.loaders.lock().await;

        LoadableRelationManyToMany::<entities::group_role::Columns>::load_many_related(
            &mut loaders.group_member,
            context,
            vec![self.id],
        )
        .await
    }
}
