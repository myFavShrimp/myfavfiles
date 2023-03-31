use sea_query::{Iden, Values};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

use std::fmt::Debug;

use crate::database::{
    actions::build_select_query,
    driver::bind_query_as,
    entities::{id_entity::IdEntity, Identifiable, TableEntity},
    relation::{ManyToManyRelation, OneToXRelation},
    PoolConnection,
};

use super::LoaderError;

pub async fn query<E>(
    conn: &mut PoolConnection,
    sql: String,
    values: Values,
) -> Result<Vec<E>, LoaderError>
where
    E: Clone + for<'r> FromRow<'r, PgRow> + Send + Unpin + Sync + TableEntity + Debug,
{
    let query = bind_query_as(sqlx::query_as::<_, E>(&sql), &values);
    match query.fetch_all(conn).await {
        Ok(rows) => Ok(rows.iter().fold(Vec::new(), |mut acc, item| {
            acc.push(item.clone());

            acc
        })),
        Err(e) => Err(e)?,
    }
}

pub async fn query_ids(
    conn: &mut PoolConnection,
    sql: String,
    values: Values,
) -> Result<Vec<IdEntity>, LoaderError> {
    query(conn, sql, values).await
}

pub async fn find_many<E>(
    db_conn: &mut PoolConnection,
    ids: Option<Vec<Uuid>>,
) -> Result<Vec<E>, LoaderError>
where
    E: Clone
        + for<'r> FromRow<'r, PgRow>
        + Send
        + Unpin
        + Identifiable
        + Sync
        + TableEntity
        + Identifiable
        + Debug,
    <E as TableEntity>::ColumnsEnum: Iden + Send + 'static,
{
    let columns = E::all_columns();
    let id_column = E::id_column();
    let table = E::table();
    let (sql, values) = build_select_query(columns, table, id_column, ids);

    query(db_conn, sql, values).await
}

pub async fn find_many_ids_related<A, B>(
    db_conn: &mut PoolConnection,
    a_id: Uuid,
) -> Result<Vec<Uuid>, LoaderError>
where
    A: OneToXRelation<B> + Identifiable,
    A::ColumnsEnum: Iden + 'static,
    B: Identifiable,
    B::ColumnsEnum: Iden + 'static,
{
    let relation_id_column = <A as OneToXRelation<B>>::target_relation_id_column();
    let columns = vec![B::id_column()];
    let table = B::table();
    let (sql, values) = build_select_query(columns, table, relation_id_column, Some(vec![a_id]));

    let id_entities = query_ids(db_conn, sql, values).await?;

    Ok(id_entities.iter().map(IdEntity::id).collect())
}

pub async fn find_many_ids_related_associative<A, B, R>(
    db_conn: &mut PoolConnection,
    a_id: Uuid,
) -> Result<Vec<Uuid>, LoaderError>
where
    A: ManyToManyRelation<B, R> + Identifiable,
    B: ManyToManyRelation<A, R> + Identifiable,
    R: Clone + for<'r> FromRow<'r, PgRow> + Send + Unpin + Sync + TableEntity + Debug,
    R::ColumnsEnum: Iden + 'static,
{
    let relation_id_column = <A as ManyToManyRelation<B, R>>::own_relation_id_column();
    let columns = R::all_columns();
    let table = R::table();
    let (sql, values) = build_select_query(columns, table, relation_id_column, Some(vec![a_id]));

    let association_entities: Vec<R> = query(db_conn, sql, values).await?;

    Ok(association_entities
        .into_iter()
        .map(<A as ManyToManyRelation<B, R>>::other_entity_id)
        .collect())
}
