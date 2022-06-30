use uuid::Uuid;

use super::{Cache, LoadableRelationOneToMany, Loader};
use crate::database::entities;

#[derive(Default)]
pub struct GroupFileShare {
    pub cache: Cache<Uuid, entities::group_file_share::Entity>,
}

#[async_trait::async_trait]
impl Loader for GroupFileShare {
    type LoadableEntity = entities::group_file_share::Entity;

    fn cache(&mut self) -> Cache<Uuid, entities::group_file_share::Entity> {
        self.cache.clone()
    }
}

impl LoadableRelationOneToMany<entities::user::Columns> for GroupFileShare {}

impl LoadableRelationOneToMany<entities::group::Columns> for GroupFileShare {}
