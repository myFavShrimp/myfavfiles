use uuid::Uuid;

use super::{Cache, Loadable};
use crate::database::entities;

#[derive(Default)]
pub struct GroupLoader {
    pub cache: Cache<Uuid, entities::group::Entity>,
}

#[async_trait::async_trait]
impl Loadable<entities::group::Entity, entities::group::Columns> for GroupLoader {
    fn cache(&mut self) -> Cache<Uuid, entities::group::Entity> {
        self.cache.clone()
    }
}
