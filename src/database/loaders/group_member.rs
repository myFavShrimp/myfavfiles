use uuid::Uuid;

use super::{Cache, Loadable, LoadableRelationOneToOne};
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
    LoadableRelationOneToOne<
        entities::group_member::Entity,
        entities::group_member::Columns,
        entities::user::Columns,
    > for GroupMemberLoader
{
    fn related_column() -> entities::group_member::Columns {
        entities::group_member::Columns::UserId
    }
}

impl
    LoadableRelationOneToOne<
        entities::group_member::Entity,
        entities::group_member::Columns,
        entities::group::Columns,
    > for GroupMemberLoader
{
    fn related_column() -> entities::group_member::Columns {
        entities::group_member::Columns::GroupId
    }
}
