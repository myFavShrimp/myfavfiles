use std::{collections::HashMap, sync::Arc};

use juniper::futures::lock::Mutex;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, Value, Values};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

use self::sea_query_driver_postgres::bind_query_as;

use super::Context;
sea_query::sea_query_driver_postgres!();

pub mod group;
pub mod user;

#[derive(Default)]
pub struct Loaders {
    pub user: user::UserLoader,
    pub group: group::GroupLoader,
}

pub type Cache<I, E> = Arc<Mutex<HashMap<I, Arc<E>>>>;

#[async_trait::async_trait]
pub trait Loadable<LoadableType, ColumnType>
where
    LoadableType: Clone + for<'r> FromRow<'r, PgRow> + Send + Unpin + GetId + Sync,
    sea_query::Value: From<Uuid>,
    ColumnType: Iden + Send + 'static,
{
    fn get_cache(&mut self) -> Cache<Uuid, LoadableType>;

    fn get_query_columns() -> (Vec<ColumnType>, ColumnType, ColumnType);

    async fn load_many(&mut self, ctx: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<LoadableType>> {
        let mut results = Vec::new();
        let mut _cache = self.get_cache();
        let mut cache = _cache.lock().await;

        let ids_to_load = match ids {
            Some(ids) => Some(ids.iter().fold(Vec::new(), |mut acc, id| {
                if let Some(item) = cache.get(id) {
                    results.push(item.clone())
                } else {
                    acc.push(id.clone());
                }

                acc
            })),
            None => None,
        };

        let (columns, id_column, table) = Self::get_query_columns();
        let (sql, values) = build_select_query(columns, table, id_column, ids_to_load);

        Self::query(ctx, sql, values).await.iter().for_each(|item| {
            let arc_item = Arc::new(item.clone());
            cache.insert(arc_item.get_id(), arc_item.clone());
            results.push(arc_item);
        });

        results
    }

    async fn query(ctx: &Context, sql: String, values: Values) -> Vec<LoadableType> {
        let mut conn = ctx
            .app_state
            .clone()
            .database_connection
            .acquire()
            .await
            .unwrap();
        let query = bind_query_as(sqlx::query_as::<_, LoadableType>(&sql), &values);
        if let Ok(rows) = query.fetch_all(&mut conn).await {
            rows.iter().fold(Vec::new(), |mut acc, item| {
                acc.push(item.clone());

                acc
            })
        } else {
            Vec::new()
        }
    }
}

fn build_select_query<E, I>(
    columns: Vec<E>,
    table: E,
    id_column: E,
    ids_to_load: Option<Vec<I>>,
) -> (String, Values)
where
    E: Iden + 'static,
    I: Into<Value>,
{
    match ids_to_load {
        Some(ids_to_load) => Query::select()
            .columns(columns)
            .from(table)
            .and_where(Expr::col(id_column).is_in(ids_to_load))
            .build(PostgresQueryBuilder),
        None => Query::select()
            .columns(columns)
            .from(table)
            .build(PostgresQueryBuilder),
    }
}

pub trait GetId {
    fn get_id(&self) -> Uuid;
}
