use std::{collections::HashMap, sync::Arc};

use juniper::futures::lock::Mutex;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, Value, Values};
use sqlx::{postgres::PgRow, Column, FromRow};
use uuid::Uuid;

use crate::handlers::graphql::Context;

use self::sea_query_driver_postgres::bind_query_as;

use super::entities::{IdEntity, RelationOneToOne, TableEntity};

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
pub trait Loadable<LoadableType, ColumnType>
where
    LoadableType: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity<ColumnType>,
    sea_query::Value: From<Uuid>,
    ColumnType: Iden + Send + 'static,
{
    fn cache(&mut self) -> Cache<Uuid, LoadableType>;

    async fn load_many(&mut self, ctx: &Context, ids: Option<Vec<Uuid>>) -> Vec<Arc<LoadableType>> {
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

        let columns = LoadableType::all_columns();
        let id_column = LoadableType::id_column();
        let table = LoadableType::table();
        let (sql, values) = build_select_query(columns, table, id_column, ids_to_load);

        Self::query(ctx, sql, values).await.iter().for_each(|item| {
            let arc_item = Arc::new(item.clone());
            cache.insert(arc_item.id(), arc_item.clone());
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

fn build_select_query<ColumnType, IdType, ColumnTypeId>(
    columns: Vec<ColumnType>,
    table: ColumnType,
    id_column: ColumnTypeId,
    ids_to_load: Option<Vec<IdType>>,
) -> (String, Values)
where
    ColumnType: Iden + 'static,
    IdType: Into<Value>,
    ColumnTypeId: Iden + 'static,
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

// fn build_relational_select_query<ColumnType, EntityType>() -> (String, Values)
// where
//     ColumnType: Iden + 'static,
//     EntityType: TableEntity<ColumnType>,
// {
//     Query::select()
//         .columns(vec![EntityType::id_column()])
//         .from(EntityType::table())
//         .and_where(Expr::col(id_column).is_in(ids_to_load))
//         .build(PostgresQueryBuilder)
// }

pub trait Identifiable {
    fn id(&self) -> Uuid;
}

#[async_trait::async_trait]
pub trait LoadableRelationOneToOne<LoadableType, ColumnType, OtherColumnType>:
    Loadable<LoadableType, ColumnType>
where
    LoadableType: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity<ColumnType>,
    ColumnType: Iden + Send + 'static,
    OtherColumnType: Iden + Send + 'static,
{
    fn related_column() -> ColumnType;

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

    async fn load_many_related(&mut self, ctx: &Context, ids: Vec<Uuid>) -> Vec<Arc<LoadableType>> {
        let id_column = LoadableType::id_column();
        let table = LoadableType::table();

        let (sql, values) =
            build_select_query(vec![id_column], table, Self::related_column(), Some(ids));
        let relational_ids = Self::query_ids(ctx, sql, values).await;

        self.load_many(ctx, Some(relational_ids)).await
    }
}

// #[async_trait::async_trait]
// pub trait LoadableRelationOneToOne<
//     LoadableType,
//     ColumnType,
//     OtherLoadableType,
//     OtherColumnType,
// >: Loadable<LoadableType, ColumnType> + RelationOneToOne<OtherEntity, OtherColumnType, ColumnType> where
//     OtherEntity: Loadable<OtherLoadableType, OtherColumnType> + TableEntity<OtherColumnType>,
//     LoadableType: Clone
//         + for<'r> FromRow<'r, PgRow>
//         + Send
//         + Unpin
//         + Identifiable
//         + Sync
//         + TableEntity<ColumnType>,
//     OtherLoadableType: Clone
//         + for<'r> FromRow<'r, PgRow>
//         + Send
//         + Unpin
//         + Identifiable
//         + Sync
//         + TableEntity<OtherColumnType>,
//     ColumnType: Iden + Send + 'static,
//     OtherColumnType: Iden + Send + 'static,
// {
//     async fn query_ids(ctx: &Context, sql: String, values: Values) -> Vec<Uuid> {
//         let mut conn = ctx
//             .app_state
//             .clone()
//             .database_connection
//             .acquire()
//             .await
//             .unwrap();
//         let query = bind_query_as(sqlx::query_as::<_, IdEntity>(&sql), &values);
//         if let Ok(rows) = query.fetch_all(&mut conn).await {
//             rows.iter().fold(Vec::new(), |mut acc, item| {
//                 acc.push(item.id.clone());

//                 acc
//             })
//         } else {
//             Vec::new()
//         }
//     }

//     async fn load_many_related<IDs>(&mut self, ctx: &Context, ids: Vec<Uuid>) -> Vec<Arc<LoadableType>> {
//         let id_column = LoadableType::id_column();
//         let table = LoadableType::table();

//         let (sql, values) = build_select_query(vec![id_column], table, Self::get_relation_id_column(), Some(ids));
//         let relational_ids = Self::query_ids(ctx, sql, values).await;

//         self.load_many(ctx, Some(relational_ids)).await
//     }
// }
