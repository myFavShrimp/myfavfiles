use super::{LoadableRelationOneToMany, Loader};
use crate::database::{
    cache::{Cache, HasCache},
    entities,
};

#[derive(Default)]
pub struct GroupFileShare {
    pub cache: Cache<entities::group_file_share::Entity>,
}

impl HasCache<entities::group_file_share::Entity> for GroupFileShare {
    fn cache(&mut self) -> Cache<entities::group_file_share::Entity> {
        self.cache.clone()
    }
}

#[async_trait::async_trait]
impl Loader for GroupFileShare {
    type LoadableEntity = entities::group_file_share::Entity;
}

impl LoadableRelationOneToMany<entities::user::Columns> for GroupFileShare {}

impl LoadableRelationOneToMany<entities::group::Columns> for GroupFileShare {}
