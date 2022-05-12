use std::{collections::HashMap, sync::Arc};

use juniper::futures::lock::Mutex;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, Value, Values};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

use crate::handlers::graphql::Context;

use self::sea_query_driver_postgres::bind_query_as;

use super::entities::{IdEntity, RelationColumn, TableEntity};

sea_query::sea_query_driver_postgres!();

pub mod group;
pub mod group_member;
pub mod user;

#[derive(Default)]
pub struct Loaders {
    pub user: user::UserLoader,
    pub group: group::GroupLoader,
    pub group_member: group_member::GroupMemberLoader,
}

pub type Cache<I, E> = Arc<Mutex<HashMap<I, Arc<E>>>>;

#[async_trait::async_trait]
pub trait Loader<LoadableEntity, ColumnsEnum>
where
    LoadableEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity<ColumnsEnum>,
    sea_query::Value: From<Uuid>,
    ColumnsEnum: Iden + Send + 'static,
{
    fn cache(&mut self) -> Cache<Uuid, LoadableEntity>;

    async fn load_many(&mut self, ctx: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<LoadableEntity>> {
        let mut results = Vec::new();
        let mut _cache = self.cache();
        let mut cache = _cache.lock().await;

        let ids_to_load = ids.map(|ids| {
            ids.iter().fold(Vec::new(), |mut acc, id| {
                if let Some(item) = cache.get(id) {
                    println!("using cache");
                    results.push(item.clone())
                } else {
                    println!("no cache");
                    acc.push(*id);
                }

                acc
            })
        });

        let columns = LoadableEntity::all_columns();
        let id_column = LoadableEntity::id_column();
        let table = LoadableEntity::table();
        let (sql, values) = build_select_query(columns, table, id_column, ids_to_load);

        Self::query(ctx, sql, values).await.iter().for_each(|item| {
            let arc_item = Arc::new(item.clone());
            cache.insert(arc_item.id(), arc_item.clone());
            results.push(arc_item);
        });

        results
    }

    async fn query(ctx: &Context, sql: String, values: Values) -> Vec<LoadableEntity> {
        let mut conn = ctx
            .app_state
            .clone()
            .database_connection
            .acquire()
            .await
            .unwrap();
        let query = bind_query_as(sqlx::query_as::<_, LoadableEntity>(&sql), &values);
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

fn build_select_query<ColumnsEnum, IdType, ColumnsEnumId>(
    columns: Vec<ColumnsEnum>,
    table: ColumnsEnum,
    id_column: ColumnsEnumId,
    ids_to_load: Option<Vec<IdType>>,
) -> (String, Values)
where
    ColumnsEnum: Iden + 'static,
    IdType: Into<Value>,
    ColumnsEnumId: Iden + 'static,
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

pub trait Identifiable {
    fn id(&self) -> Uuid;
}

#[async_trait::async_trait]
pub trait LoadableRelationOneToMany<LoadableEntity, ColumnsEnum, OtherColumnsEnum>:
    Loader<LoadableEntity, ColumnsEnum>
where
    LoadableEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity<ColumnsEnum>,
    ColumnsEnum: Iden + Send + 'static + RelationColumn<OtherColumnsEnum, ColumnsEnum>,
    OtherColumnsEnum: Iden + Send + 'static,
{
    async fn query_ids(ctx: &Context, sql: String, values: Values) -> Vec<Uuid> {
        let mut conn = ctx
            .app_state
            .clone()
            .database_connection
            .acquire()
            .await
            .unwrap();
        let query = bind_query_as(sqlx::query_as::<_, IdEntity>(&sql), &values);
        if let Ok(rows) = query.fetch_all(&mut conn).await {
            rows.iter().fold(Vec::new(), |mut acc, item| {
                acc.push(item.id.clone());

                acc
            })
        } else {
            Vec::new()
        }
    }

    async fn load_many_related(&mut self, ctx: &Context, ids: Vec<Uuid>) -> Vec<Arc<LoadableEntity>> {
        let id_column = LoadableEntity::id_column();
        let table = LoadableEntity::table();

        let (sql, values) = build_select_query(
            vec![id_column],
            table,
            ColumnsEnum::get_relation_id_column(),
            Some(ids),
        );
        let relational_ids = Self::query_ids(ctx, sql, values).await;

        self.load_many(ctx, Some(relational_ids)).await
    }
}


#[async_trait::async_trait]
pub trait LoadableRelationManyToMany<LoadableEntity, ColumnsEnum, OtherLoadableEntity, OtherColumnsEnum>: Loader<LoadableEntity, ColumnsEnum>
where
    LoadableEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity<ColumnsEnum>,
    ColumnsEnum: Iden + Send + 'static + RelationColumn<OtherColumnsEnum, ColumnsEnum>,
    OtherLoadableEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity<OtherColumnsEnum>,
    OtherColumnsEnum: Iden + Send + 'static,
    {
    async fn load_many_related(&mut self, ctx: &Context, ids: Vec<Uuid>) -> Vec<Arc<LoadableEntity>> {
        panic!("Not implemented!");
    }
}
