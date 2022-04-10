use std::sync::Arc;

use uuid::Uuid;

use super::{Cache, Loadable};
use crate::entities;

#[derive(Default)]
pub struct UserLoader {
    pub cache: Cache<Uuid, Arc<entities::user::Entity>>,
}

#[async_trait::async_trait]
impl Loadable<entities::user::Entity, entities::user::Columns> for UserLoader {
    fn get_cache(&mut self) -> Cache<Uuid, Arc<entities::user::Entity>> {
        self.cache.clone()
    }

    fn get_query_columns() -> (
        Vec<entities::user::Columns>,
        entities::user::Columns,
        entities::user::Columns,
    ) {
        (
            vec![
                entities::user::Columns::Id,
                entities::user::Columns::Name,
                entities::user::Columns::Password,
                entities::user::Columns::IsAdmin,
            ],
            entities::user::Columns::Id,
            entities::user::Columns::Table,
        )
    }
}
