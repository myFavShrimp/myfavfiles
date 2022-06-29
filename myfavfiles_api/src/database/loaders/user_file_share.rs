use uuid::Uuid;

use super::{Cache, LoadableRelationOneToMany, Loader, LoadableRelationManyToMany};
use crate::database::entities;

#[derive(Default)]
pub struct UserFileShare {
    pub cache: Cache<Uuid, entities::user_file_share::Entity>,
}

#[async_trait::async_trait]
impl Loader for UserFileShare {
    type LoadableEntity = entities::user_file_share::Entity;

    fn cache(&mut self) -> Cache<Uuid, entities::user_file_share::Entity> {
        self.cache.clone()
    }
}

impl LoadableRelationOneToMany<entities::user::Columns> for UserFileShare {}
