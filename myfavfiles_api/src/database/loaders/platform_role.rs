use uuid::Uuid;

use super::{LoadableRelationManyToMany, Loader};
use crate::database::{
    cache::{Cache, HasCache},
    entities,
};

#[derive(Default)]
pub struct PlatformRoleLoader {
    pub cache: Cache<entities::platform_role::Entity>,
}

impl HasCache<entities::platform_role::Entity> for PlatformRoleLoader {
    fn cache(&mut self) -> Cache<entities::platform_role::Entity> {
        self.cache.clone()
    }
}

#[async_trait::async_trait]
impl Loader for PlatformRoleLoader {
    type LoadableEntity = entities::platform_role::Entity;
}

impl LoadableRelationManyToMany<entities::user::Columns> for PlatformRoleLoader {
    type AssociationEntity = entities::user_role::Entity;

    fn associated_id(entity: Self::AssociationEntity) -> Uuid {
        entity.user_id
    }
}
