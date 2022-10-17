use std::{fmt::Debug, sync::Arc};

use sea_query::Iden;
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

use crate::database::{
    cache::Cache,
    entities::{Identifiable, TableEntity},
    PoolConnection,
};

use super::cacheless::find_many;

pub async fn find_many_cached<E>(
    cache: Cache<E>,
    db_conn: &mut PoolConnection,
    ids: Option<Vec<Uuid>>,
) -> Vec<Arc<E>>
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
    sea_query::Value: From<Uuid>,
{
    let cached_ids = cache.all_cached().await;
    let mut results = cache.get_all(&cached_ids).await;

    let ids_to_load = ids.map(|ids| {
        ids.into_iter()
            .filter(|id| !cached_ids.contains(id))
            .collect()
    });

    let cache_lock = cache.cache_map();
    let mut cache_map = cache_lock.lock().await;

    find_many::<E>(db_conn, ids_to_load)
        .await
        .iter()
        .for_each(|item| {
            let arc_item = Arc::new(item.clone());
            cache_map.insert(arc_item.id(), arc_item.clone());
            results.push(arc_item);
        });
    results
}