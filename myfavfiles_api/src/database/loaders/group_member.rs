use uuid::Uuid;

use super::{LoadableRelationManyToMany, LoadableRelationOneToMany, Loader};
use crate::database::{entities, cache::{Cache, HasCache}};

#[derive(Default)]
pub struct GroupMemberLoader {
    pub cache: Cache<entities::group_member::Entity>,
}

impl HasCache<entities::group_member::Entity> for GroupMemberLoader {
    fn cache(&mut self) -> Cache<entities::group_member::Entity> {
        self.cache.clone()
    }
}

#[async_trait::async_trait]
impl Loader for GroupMemberLoader {
    type LoadableEntity = entities::group_member::Entity;
}

impl LoadableRelationOneToMany<entities::user::Columns> for GroupMemberLoader {}

impl LoadableRelationOneToMany<entities::group::Columns> for GroupMemberLoader {}

impl LoadableRelationManyToMany<entities::group_role::Columns> for GroupMemberLoader {
    type AssociationEntity = entities::group_member_role::Entity;

    fn associated_id(entity: Self::AssociationEntity) -> Uuid {
        entity.group_role_id
    }
}
