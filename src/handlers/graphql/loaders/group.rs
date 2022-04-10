use std::sync::Arc;

use uuid::Uuid;

use crate::entities;
use super::{Loadable, Cache};


#[derive(Default)]
pub struct GroupLoader {
    pub cache: Cache<Uuid, Arc<entities::group::Entity>>,
}

#[async_trait::async_trait]
impl Loadable<entities::group::Entity, entities::group::Columns> for GroupLoader {
    fn get_cache(&mut self) -> Cache<Uuid, Arc<entities::group::Entity> >  {
        self.cache.clone()
    }

    fn get_query_columns() -> (Vec<entities::group::Columns>, entities::group::Columns, entities::group::Columns) {
        (
            vec![
                entities::group::Columns::Id,
                entities::group::Columns::Name,
            ], entities::group::Columns::Id,
            entities::group::Columns::Table
        )
    }
}
