use uuid::Uuid;

use super::{Cache, Loader, LoadableRelationOneToMany};
use crate::database::entities;

#[derive(Default)]
pub struct GroupMemberLoader {
    pub cache: Cache<Uuid, entities::group_member::Entity>,
}

#[async_trait::async_trait]
impl Loader<entities::group_member::Entity>
    for GroupMemberLoader
{
    fn cache(&mut self) -> Cache<Uuid, entities::group_member::Entity> {
        self.cache.clone()
    }
}

impl
    LoadableRelationOneToMany<
        entities::group_member::Entity,
        entities::user::Columns,
    > for GroupMemberLoader
{
}

impl
    LoadableRelationOneToMany<
        entities::group_member::Entity,
        entities::group::Columns,
    > for GroupMemberLoader
{
}
