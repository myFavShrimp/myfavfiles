use uuid::Uuid;

use super::{Cache, Loader};
use crate::database::entities;

#[derive(Default)]
pub struct UserLoader {
    pub cache: Cache<Uuid, entities::user::Entity>,
}

#[async_trait::async_trait]
impl Loader for UserLoader {
    type LoadableEntity = entities::user::Entity;

    fn cache(&mut self) -> Cache<Uuid, entities::user::Entity> {
        self.cache.clone()
    }
}
