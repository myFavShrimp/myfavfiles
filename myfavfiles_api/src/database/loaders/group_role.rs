use uuid::Uuid;

use super::{Cache, LoadableRelationManyToMany, Loader};
use crate::database::entities;

#[derive(Default)]
pub struct GroupRoleLoader {
    pub cache: Cache<Uuid, entities::group_role::Entity>,
}

#[async_trait::async_trait]
impl Loader for GroupRoleLoader {
    type LoadableEntity = entities::group_role::Entity;

    fn cache(&mut self) -> Cache<Uuid, entities::group_role::Entity> {
        self.cache.clone()
    }
}

// impl LoadableRelationManyToMany<entities::user::Columns> for PlatformRoleLoader {
//     type AssociationEntity = entities::user_role::Entity;
//
//     fn associated_id(entity: Self::AssociationEntity) -> Uuid {
//         entity.user_id
//     }
// }
