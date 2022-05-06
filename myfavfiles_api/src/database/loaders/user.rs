use uuid::Uuid;

use super::{Cache, Loadable};
use crate::database::entities;

#[derive(Default)]
pub struct UserLoader {
    pub cache: Cache<Uuid, entities::user::Entity>,
}

#[async_trait::async_trait]
impl Loadable<entities::user::Entity, entities::user::Columns> for UserLoader {
    fn cache(&mut self) -> Cache<Uuid, entities::user::Entity> {
        self.cache.clone()
    }
}
