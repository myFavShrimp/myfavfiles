use uuid::Uuid;

use super::{LoadableRelationManyToMany, Loader};
use crate::database::{entities, cache::{Cache, HasCache}};

#[derive(Default)]
pub struct UserLoader {
    pub cache: Cache<entities::user::Entity>,
}

impl HasCache<entities::user::Entity> for UserLoader {
    fn cache(&mut self) -> Cache<entities::user::Entity> {
        self.cache.clone()
    }
}

#[async_trait::async_trait]
impl Loader for UserLoader {
    type LoadableEntity = entities::user::Entity;
}

impl LoadableRelationManyToMany<entities::platform_role::Columns> for UserLoader {
    type AssociationEntity = entities::user_role::Entity;

    fn associated_id(entity: Self::AssociationEntity) -> Uuid {
        entity.platform_role_id
    }
}
