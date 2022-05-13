use uuid::Uuid;

use super::{Cache, Loader};
use crate::database::entities;

#[derive(Default)]
pub struct GroupLoader {
    pub cache: Cache<Uuid, entities::group::Entity>,
}

#[async_trait::async_trait]
impl Loader for GroupLoader {
    type LoadableEntity = entities::group::Entity;

    fn cache(&mut self) -> Cache<Uuid, entities::group::Entity> {
        self.cache.clone()
    }
}
