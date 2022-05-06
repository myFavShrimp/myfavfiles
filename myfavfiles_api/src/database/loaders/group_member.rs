use uuid::Uuid;

use super::{Cache, Loadable, LoadableRelationOneToMany};
use crate::database::entities;

#[derive(Default)]
pub struct GroupMemberLoader {
    pub cache: Cache<Uuid, entities::group_member::Entity>,
}

#[async_trait::async_trait]
impl Loadable<entities::group_member::Entity, entities::group_member::Columns>
    for GroupMemberLoader
{
    fn cache(&mut self) -> Cache<Uuid, entities::group_member::Entity> {
        self.cache.clone()
    }
}

impl
    LoadableRelationOneToMany<
        entities::group_member::Entity,
        entities::group_member::Columns,
        entities::user::Columns,
    > for GroupMemberLoader
{
}

impl
    LoadableRelationOneToMany<
        entities::group_member::Entity,
        entities::group_member::Columns,
        entities::group::Columns,
    > for GroupMemberLoader
{
}
