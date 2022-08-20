use uuid::Uuid;

use super::{LoadableRelationManyToMany, LoadableRelationOneToMany, Loader};
use crate::database::{entities, cache::{HasCache, Cache}};

#[derive(Default)]
pub struct GroupRoleLoader {
    pub cache: Cache<entities::group_role::Entity>,
}

impl HasCache<entities::group_role::Entity> for GroupRoleLoader {
    fn cache(&mut self) -> Cache<entities::group_role::Entity> {
        self.cache.clone()
    }
}

#[async_trait::async_trait]
impl Loader for GroupRoleLoader {
    type LoadableEntity = entities::group_role::Entity;
}

impl LoadableRelationOneToMany<entities::group::Columns> for GroupRoleLoader {}

impl LoadableRelationManyToMany<entities::group_member::Columns> for GroupRoleLoader {
    type AssociationEntity = entities::group_member_role::Entity;

    fn associated_id(entity: Self::AssociationEntity) -> Uuid {
        entity.group_member_id
    }
}
