use uuid::Uuid;

use super::{Cache, Loader, LoadableRelationOneToMany};
use crate::database::entities;

#[derive(Default)]
pub struct GroupMemberLoader {
    pub cache: Cache<Uuid, entities::group_member::Entity>,
}

#[async_trait::async_trait]
impl Loader
    for GroupMemberLoader
{
    type LoadableEntity = entities::group_member::Entity;
    
    fn cache(&mut self) -> Cache<Uuid, entities::group_member::Entity> {
        self.cache.clone()
    }
}

impl
    LoadableRelationOneToMany<
        entities::user::Columns,
    > for GroupMemberLoader
{
}

impl
    LoadableRelationOneToMany<
        entities::group::Columns,
    > for GroupMemberLoader
{
}
