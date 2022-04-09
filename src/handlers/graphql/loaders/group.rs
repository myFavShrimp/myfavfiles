use std::sync::Arc;

use uuid::Uuid;

use crate::{entities, handlers::graphql::Context};
use super::{Loadable, sea_query_driver_postgres::bind_query_as, build_select_query, Cache};


#[derive(Default)]
pub struct GroupLoader {
    pub cache: Cache<Uuid, Arc<entities::group::Entity>>,
}

#[async_trait::async_trait]
impl Loadable for GroupLoader {
    type IdentifierType = Uuid;
    type LoadableType = entities::group::Entity;

    async fn load_many(&mut self, ctx: &Context, ids: Option<Vec<Self::IdentifierType>>) -> Vec<Arc<Self::LoadableType>> {
        let mut results = Vec::new();
        let mut _cache = self.get_cache();
        let mut cache = _cache.lock().await;

        let ids_to_load = match ids {
            Some(ids) => {
                Some(ids.iter().fold(Vec::new(), |mut acc, id| {
                    if let Some(item) = cache.get(id) {
                        results.push(item.clone())
                    } else {
                        acc.push(id.clone());
                    }

                    acc
                }))
            }
            None => None
        };

        let (sql, values) = build_select_query(
            vec![
                entities::group::Columns::Id,
                entities::group::Columns::Name,
            ], entities::group::Columns::Table, 
            entities::group::Columns::Id, 
            ids_to_load,
        );

        let mut conn = ctx.app_state.clone().database_connection.try_acquire().unwrap();
        let query = bind_query_as(sqlx::query_as::<_, Self::LoadableType>(&sql), &values);
        if let Ok(mut rows) = query.fetch_all(&mut conn).await {
            rows.iter_mut().for_each(|item| {
                let arc_item = Arc::new(item.clone());
                cache.insert(arc_item.id, arc_item.clone());
                results.push(arc_item);
            });
        };

        results
    }

    fn get_cache(&mut self) -> Cache<Self::IdentifierType,Arc<Self::LoadableType> >  {
        self.cache.clone()
    }
}
