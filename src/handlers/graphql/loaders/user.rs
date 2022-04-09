use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{entities, handlers::graphql::Context};
use super::{Loadable, sea_query_driver_postgres::bind_query_as, build_select_query};


#[derive(Default)]
pub struct UserLoader {
    pub cache: HashMap<Uuid, Arc<entities::user::Entity>>,
}

#[async_trait::async_trait]
impl Loadable for UserLoader {
    type IdentifierType = Uuid;
    type LoadableType = entities::user::Entity;

    async fn load_many(&mut self, ctx: &Context, ids: Option<Vec<Self::IdentifierType>>) -> Vec<Arc<Self::LoadableType>> {
        let mut results = Vec::new();

        let ids_to_load = match ids {
            Some(ids) => {
                Some(ids.iter().fold(Vec::new(), |mut acc, id| {
                    if let Some(item) = self.cache.get(id) {
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
                entities::user::Columns::Id,
                entities::user::Columns::Name,
                entities::user::Columns::Password,
                entities::user::Columns::IsAdmin,
            ], entities::user::Columns::Table, 
            entities::user::Columns::Id, 
            ids_to_load,
        );


        let mut conn = ctx.app_state.clone().database_connection.try_acquire().unwrap();
        let query = bind_query_as(sqlx::query_as::<_, Self::LoadableType>(&sql), &values);
        if let Ok(mut rows) = query.fetch_all(&mut conn).await {
            rows.iter_mut().for_each(|item| {
                let arc_item = Arc::new(item.clone());
                self.cache.insert(arc_item.id, arc_item.clone());
                results.push(arc_item);
            });
        };

        results
    }
}
