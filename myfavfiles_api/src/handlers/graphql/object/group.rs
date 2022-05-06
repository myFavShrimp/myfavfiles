use std::sync::Arc;

use juniper::graphql_object;
use uuid::Uuid;

use super::super::Context;
use crate::database::{
    entities,
    loaders::{Loadable, LoadableRelationOneToMany},
};

#[graphql_object(Context = Context, name = "Group")]
impl entities::group::Entity {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn group_member(context: &Context) -> Vec<Arc<entities::group_member::Entity>> {
        let mut loaders = context.loaders.lock().await;

        LoadableRelationOneToMany::<
            entities::group_member::Entity,
            entities::group_member::Columns,
            entities::group::Columns,
        >::load_many_related(&mut loaders.group_member, context, vec![self.id])
        .await
    }
}
