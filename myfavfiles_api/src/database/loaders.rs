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
pub trait Loader
where
    Self::LoadableEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity,
    <Self::LoadableEntity as TableEntity>::ColumnsEnum: Iden + Send + 'static,
    sea_query::Value: From<Uuid>,
{
    type LoadableEntity;

    fn cache(&mut self) -> Cache<Uuid, Self::LoadableEntity>;

    async fn load_many(&mut self, ctx: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<Self::LoadableEntity>> {
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

        let columns = Self::LoadableEntity::all_columns();
        let id_column = Self::LoadableEntity::id_column();
        let table = Self::LoadableEntity::table();
        let (sql, values) = build_select_query(columns, table, id_column, ids_to_load);

        let conn = ctx.database_connection().await.unwrap();

        Self::query(conn, sql, values).await.iter().for_each(|item| {
            let arc_item = Arc::new(item.clone());
            cache.insert(arc_item.id(), arc_item.clone());
            results.push(arc_item);
        });

        results
    }

    async fn query(mut conn: super::PoolConnection, sql: String, values: Values) -> Vec<Self::LoadableEntity> {
        let query = bind_query_as(sqlx::query_as::<_, Self::LoadableEntity>(&sql), &values);
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
pub trait LoadableRelationOneToMany<OtherColumnsEnum>:
    Loader
where
    <Self as Loader>::LoadableEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity,
    <<Self as Loader>::LoadableEntity as TableEntity>::ColumnsEnum:  Iden + Send + 'static + RelationColumn<OtherColumnsEnum>,
    OtherColumnsEnum: Iden + Send + 'static,
{
    async fn query_ids(ctx: &Context, sql: String, values: Values) -> Vec<Uuid> {
        let mut conn = ctx.database_connection().await
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

    async fn load_many_related(&mut self, ctx: &Context, ids: Vec<Uuid>) -> Vec<Arc<<Self as Loader>::LoadableEntity>> {
        let id_column = <Self as Loader>::LoadableEntity::id_column();
        let table = <Self as Loader>::LoadableEntity::table();

        let (sql, values) = build_select_query(
            vec![id_column],
            table,
            <<Self as Loader>::LoadableEntity as TableEntity>::ColumnsEnum::relation_id_column(),
            Some(ids),
        );
        let relational_ids = Self::query_ids(ctx, sql, values).await;

        self.load_many(ctx, Some(relational_ids)).await
    }
}


#[async_trait::async_trait]
pub trait LoadableRelationManyToMany<ColumnsEnum, OtherLoadableEntity, OtherColumnsEnum>: Loader
where
    <Self as Loader>::LoadableEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity,
    <<Self as Loader>::LoadableEntity as TableEntity>::ColumnsEnum:  Iden + Send + 'static + RelationColumn<OtherColumnsEnum>,
    OtherLoadableEntity: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity,
    <OtherLoadableEntity as TableEntity>::ColumnsEnum:  Iden + Send + 'static + RelationColumn<OtherColumnsEnum>,
    OtherColumnsEnum: Iden + Send + 'static,
    {
    async fn load_many_related(&mut self, _ctx: &Context, _ids: Vec<Uuid>) -> Vec<Arc<<Self as Loader>::LoadableEntity>> {
        panic!("Not implemented!");
    }
}
