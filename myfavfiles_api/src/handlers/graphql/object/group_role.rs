use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{
    entities,
    loaders::{LoadableRelationManyToMany, Loader},
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

    // async fn user(context: &Context) -> Vec<Arc<entities::user::Entity>> {
    //     let mut loaders = context.loaders.lock().await;
    //
    //     LoadableRelationManyToMany::<entities::platform_role::Columns>::load_many_related(
    //         &mut loaders.user,
    //         context,
    //         vec![self.id],
    //     )
    //     .await
    // }
}
