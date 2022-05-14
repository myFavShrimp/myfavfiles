use uuid::Uuid;

use super::{Cache, LoadableRelationManyToMany, Loader};
use crate::database::entities;

#[derive(Default)]
pub struct PlatformRoleLoader {
    pub cache: Cache<Uuid, entities::platform_role::Entity>,
}

#[async_trait::async_trait]
impl Loader for PlatformRoleLoader {
    type LoadableEntity = entities::platform_role::Entity;

    fn cache(&mut self) -> Cache<Uuid, entities::platform_role::Entity> {
        self.cache.clone()
    }
}

impl LoadableRelationManyToMany<entities::user::Columns> for PlatformRoleLoader {
    type AssociationEntity = entities::user_role::Entity;
}
