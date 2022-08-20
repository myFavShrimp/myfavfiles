use super::Loader;
use crate::database::{entities, cache::{Cache, HasCache}};

#[derive(Default)]
pub struct GroupLoader {
    pub cache: Cache<entities::group::Entity>,
}

impl HasCache<entities::group::Entity> for GroupLoader {
    fn cache(&mut self) -> Cache<entities::group::Entity> {
        self.cache.clone()
    }
}

#[async_trait::async_trait]
impl Loader for GroupLoader {
    type LoadableEntity = entities::group::Entity;
}
