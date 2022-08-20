use super::{LoadableRelationOneToMany, Loader};
use crate::database::{
    cache::{Cache, HasCache},
    entities,
};

#[derive(Default)]
pub struct UserFileShare {
    pub cache: Cache<entities::user_file_share::Entity>,
}

impl HasCache<entities::user_file_share::Entity> for UserFileShare {
    fn cache(&mut self) -> Cache<entities::user_file_share::Entity> {
        self.cache.clone()
    }
}

#[async_trait::async_trait]
impl Loader for UserFileShare {
    type LoadableEntity = entities::user_file_share::Entity;
}

impl LoadableRelationOneToMany<entities::user::Columns> for UserFileShare {}
