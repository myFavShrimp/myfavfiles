use uuid::Uuid;

use super::{Cache, LoadableRelationOneToMany, Loader};
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

impl LoadableRelationOneToMany<entities::group::Columns> for GroupRoleLoader {}
