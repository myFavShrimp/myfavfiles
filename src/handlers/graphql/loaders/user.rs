use std::{collections::HashMap, sync::Arc};

use sea_query::{Query, PostgresQueryBuilder, Expr};
use uuid::Uuid;

use crate::{entities, handlers::graphql::Context};
use super::{Loadable, sea_query_driver_postgres::bind_query_as};


#[derive(Default)]
pub struct UserLoader {
    pub cache: HashMap<Uuid, Arc<entities::user::Entity>>,
}

#[async_trait::async_trait]
impl Loadable for UserLoader {
    type IdentifierType = Uuid;
    type LoadableType = entities::user::Entity;

    async fn load_many(&mut self, ctx: &Context, ids: &[Self::IdentifierType]) -> Vec<Arc<Self::LoadableType>> {
        let mut results = Vec::new();

        let ids_to_load = ids.iter().fold(Vec::new(), |mut acc, id| {
            if let Some(item) = self.cache.get(id) {
                results.push(item.clone())
            } else {
                acc.push(id.clone());
            }

            acc
        });

        let (sql, values) = Query::select()
            .columns(vec![
                entities::user::Columns::Id,
                entities::user::Columns::Name,
                entities::user::Columns::Password,
                entities::user::Columns::IsAdmin,
                ])    
            .from(entities::user::Columns::Table)
            .and_where(Expr::col(entities::user::Columns::Id).is_in(ids_to_load))
            .build(PostgresQueryBuilder);

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
